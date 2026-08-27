//! AbortController and AbortSignal cancellation primitive (M27-007-A).
//!
//! Provides the core cancellation model shared by fetch, timer, and native capabilities,
//! enforcing exactly-once abort propagation, late-listener invocation, and reason preservation.

use std::fmt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

/// Signal cancellation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignalState {
    Active,
    Aborted { reason: String },
}

/// Maximum number of listeners allowed per AbortSignal to prevent unbounded growth/leaks (M27-007-C).
pub const MAX_ABORT_LISTENERS: usize = 1_024;

/// Callback type for registered abort listeners.
pub type AbortListener = Box<dyn Fn(&str) + Send + Sync + 'static>;

/// Core AbortSignal primitive.
#[derive(Clone)]
pub struct AbortSignalModel {
    aborted: Arc<AtomicBool>,
    reason: Arc<Mutex<Option<String>>>,
    listeners: Arc<Mutex<Vec<AbortListener>>>,
}

impl fmt::Debug for AbortSignalModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AbortSignalModel")
            .field("aborted", &self.is_aborted())
            .field("reason", &self.reason())
            .finish()
    }
}

impl AbortSignalModel {
    /// Create a new active signal.
    pub fn new() -> Self {
        AbortSignalModel {
            aborted: Arc::new(AtomicBool::new(false)),
            reason: Arc::new(Mutex::new(None)),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Create an already-aborted signal with a given reason.
    pub fn aborted_with(reason: &str) -> Self {
        AbortSignalModel {
            aborted: Arc::new(AtomicBool::new(true)),
            reason: Arc::new(Mutex::new(Some(reason.to_string()))),
            listeners: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Check if the signal is aborted.
    pub fn is_aborted(&self) -> bool {
        self.aborted.load(Ordering::Acquire)
    }

    /// Get the abort reason if aborted.
    pub fn reason(&self) -> Option<String> {
        self.reason.lock().unwrap().clone()
    }

    /// Abort the signal with an optional reason.
    /// Returns `true` if this call transitioned the signal from Active to Aborted (exactly once),
    /// or `false` if the signal was already aborted (idempotent no-op).
    pub fn abort(&self, reason: Option<&str>) -> bool {
        let was_aborted = self.aborted.swap(true, Ordering::SeqCst);
        if was_aborted {
            return false;
        }

        let r = reason
            .unwrap_or("AbortError: This operation was aborted")
            .to_string();
        *self.reason.lock().unwrap() = Some(r.clone());

        // Notify all registered listeners
        let listeners = {
            let mut guard = self.listeners.lock().unwrap();
            std::mem::take(&mut *guard)
        };

        for listener in listeners {
            listener(&r);
        }

        true
    }

    /// Add an abort listener with maximum listener capacity check (M27-007-C).
    pub fn try_add_listener<F>(&self, listener: F) -> Result<(), &'static str>
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        if self.is_aborted() {
            let r = self
                .reason()
                .unwrap_or_else(|| "AbortError: This operation was aborted".to_string());
            listener(&r);
            return Ok(());
        }

        let mut guard = self.listeners.lock().unwrap();
        if self.is_aborted() {
            drop(guard);
            let r = self
                .reason()
                .unwrap_or_else(|| "AbortError: This operation was aborted".to_string());
            listener(&r);
            return Ok(());
        }

        if guard.len() >= MAX_ABORT_LISTENERS {
            return Err("maximum abort listeners limit exceeded (1024)");
        }
        guard.push(Box::new(listener));
        Ok(())
    }

    /// Add an abort listener. If already aborted, the listener is invoked immediately (late listener semantics).
    pub fn add_listener<F>(&self, listener: F)
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        let _ = self.try_add_listener(listener);
    }

    /// Number of active pending listeners.
    pub fn listener_count(&self) -> usize {
        self.listeners.lock().unwrap().len()
    }

    /// Throw or return error if aborted (`throwIfAborted` semantics).
    pub fn throw_if_aborted(&self) -> Result<(), String> {
        if self.is_aborted() {
            Err(self
                .reason()
                .unwrap_or_else(|| "AbortError: This operation was aborted".to_string()))
        } else {
            Ok(())
        }
    }
}

impl Default for AbortSignalModel {
    fn default() -> Self {
        Self::new()
    }
}

/// Core AbortController primitive.
#[derive(Debug, Clone, Default)]
pub struct AbortControllerModel {
    signal: AbortSignalModel,
}

impl AbortControllerModel {
    pub fn new() -> Self {
        AbortControllerModel {
            signal: AbortSignalModel::new(),
        }
    }

    pub fn signal(&self) -> &AbortSignalModel {
        &self.signal
    }

    pub fn abort(&self, reason: Option<&str>) -> bool {
        self.signal.abort(reason)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicUsize;

    #[test]
    fn abort_propagates_exactly_once() {
        let controller = AbortControllerModel::new();
        let signal = controller.signal();
        assert!(!signal.is_aborted());
        assert_eq!(signal.reason(), None);

        // First abort succeeds
        assert!(controller.abort(Some("timeout")));
        assert!(signal.is_aborted());
        assert_eq!(signal.reason(), Some("timeout".to_string()));

        // Second abort is a no-op (returns false, reason unchanged)
        assert!(!controller.abort(Some("second reason")));
        assert_eq!(signal.reason(), Some("timeout".to_string()));
    }

    #[test]
    fn listeners_fired_on_abort() {
        let controller = AbortControllerModel::new();
        let signal = controller.signal();

        let count = Arc::new(AtomicUsize::new(0));
        let c1 = Arc::clone(&count);
        signal.add_listener(move |_| {
            c1.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(count.load(Ordering::SeqCst), 0);
        controller.abort(Some("cancelled"));
        assert_eq!(count.load(Ordering::SeqCst), 1);

        // Repeated abort does not fire listeners again
        controller.abort(Some("again"));
        assert_eq!(count.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn late_listeners_follow_defined_semantics_and_fire_immediately() {
        let controller = AbortControllerModel::new();
        let signal = controller.signal();
        controller.abort(Some("pre-aborted"));

        let fired = Arc::new(AtomicBool::new(false));
        let f = Arc::clone(&fired);
        signal.add_listener(move |reason| {
            assert_eq!(reason, "pre-aborted");
            f.store(true, Ordering::SeqCst);
        });

        assert!(fired.load(Ordering::SeqCst));
    }

    #[test]
    fn throw_if_aborted_behavior() {
        let signal = AbortSignalModel::new();
        assert!(signal.throw_if_aborted().is_ok());

        signal.abort(Some("custom reason"));
        assert_eq!(signal.throw_if_aborted(), Err("custom reason".to_string()));
    }

    /// M27-007-C: listener leak prevention tests.
    #[test]
    fn listeners_cleared_after_abort_and_bounded_capacity() {
        let signal = AbortSignalModel::new();
        assert_eq!(signal.listener_count(), 0);

        signal.add_listener(|_| {});
        signal.add_listener(|_| {});
        assert_eq!(signal.listener_count(), 2);

        signal.abort(Some("aborted"));
        // Listeners vector is drained/cleared upon abort
        assert_eq!(signal.listener_count(), 0);

        // Max listeners capacity check
        let signal2 = AbortSignalModel::new();
        for _ in 0..MAX_ABORT_LISTENERS {
            assert!(signal2.try_add_listener(|_| {}).is_ok());
        }
        assert_eq!(signal2.listener_count(), MAX_ABORT_LISTENERS);
        assert_eq!(
            signal2.try_add_listener(|_| {}),
            Err("maximum abort listeners limit exceeded (1024)")
        );
    }
}
