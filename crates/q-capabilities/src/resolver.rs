//! Compile-time capability dependency DAG (M27-002-A/B).
//!
//! Given the application's root requirements and the universe of
//! linked capability descriptors, compute the transitive closure of
//! required capabilities in a deterministic order (sorted by id).
//! Every edge is resolved with ADR-0029 semantics: exact version
//! match, typed `Missing`/`VersionConflict` failures. Cycles are
//! detected with an explicit path stack and rejected with a typed
//! `Cycle` error naming the full path (M27-002-B). An application
//! with no capability requirements resolves to an empty closure:
//! unrelated apps link nothing (guardrail: zero linked-capability
//! cost).

use std::collections::BTreeMap;

use crate::identity::{
    resolve_requirement, CapabilityDescriptor, CapabilityId, CapabilityRequirement, ResolveError,
};

/// The resolved dependency DAG for one application: the transitive
/// closure of descriptors the runtime must link, ordered
/// deterministically by capability id.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyDag {
    resolved: Vec<CapabilityDescriptor>,
}

impl DependencyDag {
    /// The linked set, sorted by capability id. Same inputs (in any
    /// order) always produce the same vector.
    pub fn resolved(&self) -> &[CapabilityDescriptor] {
        &self.resolved
    }

    /// Just the ids, in link order.
    pub fn ids(&self) -> Vec<&CapabilityId> {
        self.resolved.iter().map(|d| &d.requirement.id).collect()
    }

    pub fn is_empty(&self) -> bool {
        self.resolved.is_empty()
    }

    pub fn len(&self) -> usize {
        self.resolved.len()
    }
}

/// Compute the dependency closure for an application's root
/// requirements against the linked universe.
///
/// Deterministic: the result is sorted by id, so neither root order
/// nor universe order can change it. Failures are typed ADR-0029
/// errors naming the exact capability (and versions on conflict, or
/// the full cycle path on a cycle). Every failure happens at build
/// time, before any pack is produced.
pub fn resolve_closure(
    roots: &[CapabilityRequirement],
    universe: &[CapabilityDescriptor],
) -> Result<DependencyDag, ResolveError> {
    // Deterministic lookup regardless of universe order: first
    // descriptor carrying an id wins, as pinned by ADR-0029 §4 —
    // uniqueness itself is the compiler inventory's job (M27-002-C).
    let mut by_id: BTreeMap<&CapabilityId, &CapabilityDescriptor> = BTreeMap::new();
    for d in universe {
        by_id.entry(&d.requirement.id).or_insert(d);
    }

    let mut resolved: BTreeMap<&CapabilityId, &CapabilityDescriptor> = BTreeMap::new();

    // Iterative DFS with an explicit path stack for cycle detection.
    // Each frame holds the requirement being visited and the index of
    // the next dependency to process. When we exhaust a descriptor's
    // children we pop the frame and move it to `resolved`.
    //
    // `in_path` maps ids on the current DFS path to their position
    // in `path_ids`, giving O(1) back-edge detection and a
    // human-readable path for the Cycle error.
    let mut in_path: BTreeMap<&CapabilityId, usize> = BTreeMap::new();
    let mut path_ids: Vec<CapabilityId> = Vec::new();
    // DFS stack: (requirement being visited, dep_index already processed)
    let mut stack: Vec<(&CapabilityRequirement, usize)> = Vec::new();
    // Roots to process; BTreeMap sort dominates final output order.
    let mut pending: Vec<&CapabilityRequirement> = roots.iter().collect();

    'outer: loop {
        // Drain the pending queue until the DFS stack is non-empty.
        while stack.is_empty() {
            match pending.pop() {
                None => break 'outer,
                Some(req) => {
                    if resolved.contains_key(&req.id) {
                        continue;
                    }
                    if let Some(&idx) = in_path.get(&req.id) {
                        let mut path = path_ids[idx..].to_vec();
                        path.push(req.id.clone());
                        return Err(ResolveError::Cycle { path });
                    }
                    // Resolve and version-check before pushing.
                    let linked = match by_id.get(&req.id) {
                        Some(d) => *d,
                        None => return Err(ResolveError::Missing { id: req.id.clone() }),
                    };
                    resolve_requirement(std::slice::from_ref(linked), req)?;
                    in_path.insert(&linked.requirement.id, path_ids.len());
                    path_ids.push(linked.requirement.id.clone());
                    stack.push((req, 0));
                }
            }
        }

        let (req, dep_idx) = stack.last_mut().unwrap();
        let req_id = req.id.clone();
        let linked = *by_id.get(&req_id).unwrap(); // validated on push
        if *dep_idx < linked.dependencies.len() {
            let dep = &linked.dependencies[*dep_idx];
            *dep_idx += 1;
            if resolved.contains_key(&dep.id) {
                continue;
            }
            if let Some(&idx) = in_path.get(&dep.id) {
                let mut path = path_ids[idx..].to_vec();
                path.push(dep.id.clone());
                return Err(ResolveError::Cycle { path });
            }
            let dep_linked = match by_id.get(&dep.id) {
                Some(d) => *d,
                None => return Err(ResolveError::Missing { id: dep.id.clone() }),
            };
            resolve_requirement(std::slice::from_ref(dep_linked), dep)?;
            in_path.insert(&dep_linked.requirement.id, path_ids.len());
            path_ids.push(dep_linked.requirement.id.clone());
            stack.push((dep, 0));
        } else {
            // All children of this node are committed; commit the node.
            stack.pop();
            resolved.insert(&linked.requirement.id, linked);
            in_path.remove(&req_id);
            path_ids.pop();
        }
    }

    Ok(DependencyDag {
        resolved: resolved.into_values().cloned().collect(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identity::CapabilityVersion;

    fn req(id: &str, version: u32) -> CapabilityRequirement {
        CapabilityRequirement {
            id: CapabilityId::parse(id).unwrap(),
            version: CapabilityVersion(version),
        }
    }

    fn desc(id: &str, version: u32, deps: &[(&str, u32)]) -> CapabilityDescriptor {
        CapabilityDescriptor {
            requirement: req(id, version),
            dependencies: deps.iter().map(|(i, v)| req(i, *v)).collect(),
        }
    }

    #[test]
    fn transitive_closure_includes_all_levels() {
        let universe = [
            desc(
                "runtime:abort",
                1,
                &[("runtime:text", 1), ("runtime:url", 1)],
            ),
            desc("runtime:text", 1, &[]),
            desc("runtime:url", 1, &[("runtime:text", 1)]),
            // present but unreachable: must NOT enter the closure
            desc("runtime:crypto", 1, &[]),
        ];
        let dag = resolve_closure(&[req("runtime:abort", 1)], &universe).unwrap();
        assert_eq!(
            dag.ids().iter().map(|i| i.as_str()).collect::<Vec<_>>(),
            vec!["runtime:abort", "runtime:text", "runtime:url"]
        );
        assert_eq!(dag.len(), 3);
    }

    #[test]
    fn closure_is_deterministic_regardless_of_input_order() {
        let universe_a = [
            desc("runtime:url", 1, &[("runtime:text", 1)]),
            desc("runtime:text", 1, &[]),
            desc("runtime:abort", 1, &[("runtime:url", 1)]),
        ];
        let universe_b = {
            let mut v = universe_a.clone();
            v.reverse();
            v
        };
        let roots = [req("runtime:abort", 1), req("runtime:text", 1)];
        let roots_reversed = [req("runtime:text", 1), req("runtime:abort", 1)];
        let a = resolve_closure(&roots, &universe_a).unwrap();
        let b = resolve_closure(&roots_reversed, &universe_b).unwrap();
        assert_eq!(a, b);
        assert_eq!(format!("{a:?}"), format!("{b:?}"));
    }

    #[test]
    fn diamond_dependency_resolves_to_one_entry() {
        let universe = [
            desc(
                "runtime:abort",
                1,
                &[("runtime:text", 1), ("runtime:url", 1)],
            ),
            desc("runtime:url", 1, &[("runtime:text", 1)]),
            desc("runtime:text", 1, &[]),
        ];
        let dag = resolve_closure(&[req("runtime:abort", 1)], &universe).unwrap();
        assert_eq!(
            dag.ids()
                .iter()
                .filter(|i| i.as_str() == "runtime:text")
                .count(),
            1
        );
        assert_eq!(dag.len(), 3);
    }

    #[test]
    fn duplicate_roots_dedupe() {
        let universe = [desc("runtime:text", 1, &[])];
        let roots = [req("runtime:text", 1), req("runtime:text", 1)];
        let dag = resolve_closure(&roots, &universe).unwrap();
        assert_eq!(dag.len(), 1);
    }

    #[test]
    fn empty_roots_resolve_to_empty_closure() {
        // guardrail: an app that requires nothing links nothing
        let universe = [desc("runtime:text", 1, &[]), desc("runtime:url", 1, &[])];
        let dag = resolve_closure(&[], &universe).unwrap();
        assert!(dag.is_empty());
        assert_eq!(dag.resolved(), &[] as &[CapabilityDescriptor]);
    }

    #[test]
    fn missing_root_capability_fails_typed() {
        let universe = [desc("runtime:text", 1, &[])];
        assert_eq!(
            resolve_closure(&[req("runtime:fetch", 1)], &universe),
            Err(ResolveError::Missing {
                id: CapabilityId::parse("runtime:fetch").unwrap()
            })
        );
    }

    #[test]
    fn missing_transitive_dependency_fails_typed() {
        let universe = [desc("runtime:abort", 1, &[("runtime:text", 1)])];
        let err = resolve_closure(&[req("runtime:abort", 1)], &universe).unwrap_err();
        assert_eq!(
            err,
            ResolveError::Missing {
                id: CapabilityId::parse("runtime:text").unwrap()
            }
        );
    }

    #[test]
    fn version_conflict_on_any_edge_fails_typed_with_versions() {
        // root edge conflicts
        let universe = [desc("runtime:text", 2, &[])];
        assert_eq!(
            resolve_closure(&[req("runtime:text", 1)], &universe),
            Err(ResolveError::VersionConflict {
                id: CapabilityId::parse("runtime:text").unwrap(),
                required: CapabilityVersion(1),
                linked: CapabilityVersion(2),
            })
        );
        // transitive edge conflicts: abort needs url v1, universe links url v2
        let universe2 = [
            desc("runtime:abort", 1, &[("runtime:url", 1)]),
            desc("runtime:url", 2, &[]),
        ];
        assert_eq!(
            resolve_closure(&[req("runtime:abort", 1)], &universe2),
            Err(ResolveError::VersionConflict {
                id: CapabilityId::parse("runtime:url").unwrap(),
                required: CapabilityVersion(1),
                linked: CapabilityVersion(2),
            })
        );
    }

    #[test]
    fn cycle_is_rejected_with_typed_error_naming_path() {
        // a->b->a mutual cycle (was: walk_terminates_on_cycles in A,
        // now flipped to a build error per M27-002-B)
        let universe = [
            desc("runtime:a", 1, &[("runtime:b", 1)]),
            desc("runtime:b", 1, &[("runtime:a", 1)]),
        ];
        let err = resolve_closure(&[req("runtime:a", 1)], &universe).unwrap_err();
        assert!(
            matches!(err, ResolveError::Cycle { .. }),
            "expected Cycle, got {err:?}"
        );
        let msg = err.to_string();
        assert!(
            msg.contains("runtime:a") && msg.contains("runtime:b"),
            "{msg}"
        );
    }

    #[test]
    fn self_cycle_is_rejected() {
        let universe = [desc("runtime:a", 1, &[("runtime:a", 1)])];
        let err = resolve_closure(&[req("runtime:a", 1)], &universe).unwrap_err();
        assert!(matches!(err, ResolveError::Cycle { path } if !path.is_empty()));
    }

    #[test]
    fn longer_cycle_path_names_all_members() {
        // a -> b -> c -> a
        let universe = [
            desc("runtime:a", 1, &[("runtime:b", 1)]),
            desc("runtime:b", 1, &[("runtime:c", 1)]),
            desc("runtime:c", 1, &[("runtime:a", 1)]),
        ];
        let err = resolve_closure(&[req("runtime:a", 1)], &universe).unwrap_err();
        if let ResolveError::Cycle { path } = &err {
            let strs: Vec<&str> = path.iter().map(|i| i.as_str()).collect();
            assert!(strs.contains(&"runtime:a"), "{strs:?}");
            assert!(strs.contains(&"runtime:b"), "{strs:?}");
        } else {
            panic!("expected Cycle, got {err:?}");
        }
    }

    #[test]
    fn cycle_error_message_contains_arrow_path() {
        let universe = [
            desc("runtime:a", 1, &[("runtime:b", 1)]),
            desc("runtime:b", 1, &[("runtime:a", 1)]),
        ];
        let msg = resolve_closure(&[req("runtime:a", 1)], &universe)
            .unwrap_err()
            .to_string();
        assert!(msg.contains("->"), "expected -> in cycle message: {msg}");
    }

    #[test]
    fn deterministic_first_descriptor_wins_on_duplicate_ids() {
        // ADR-0029 §4: the first descriptor carrying the id decides;
        // duplicates themselves are the inventory's problem
        // (M27-002-C). Pinned so the lookup rule cannot drift.
        let universe = [desc("runtime:text", 1, &[]), desc("runtime:text", 2, &[])];
        let dag = resolve_closure(&[req("runtime:text", 1)], &universe).unwrap();
        assert_eq!(dag.resolved()[0].requirement.version, CapabilityVersion(1));
    }
}
