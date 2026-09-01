//! Pack-embedded source map: maps generated bundle locations back to the
//! original TypeScript sources for diagnostics (M1 requirement: a
//! TypeScript-originated exception must identify a useful original location).

use std::sync::Arc;

use q_engine_quickjs::SourceMapper;

pub struct SourcemapMapper {
    map: sourcemap::SourceMap,
}

impl SourcemapMapper {
    pub fn parse(json: &str) -> Result<SourcemapMapper, String> {
        let map = sourcemap::SourceMap::from_reader(json.as_bytes())
            .map_err(|e| format!("source map parse: {e}"))?;
        Ok(SourcemapMapper { map })
    }

    /// QuickJS columns are 0-based in rquickjs stack strings; sourcemap crate
    /// tokens use 0-based line and column as well.
    pub fn lookup(&self, line: u32, col: u32) -> Option<q_engine::OriginalLocation> {
        let token = self.map.lookup_token(line.saturating_sub(1), col)?;
        Some(q_engine::OriginalLocation {
            source: token.get_source().unwrap_or("unknown").to_string(),
            line: token.get_src_line() + 1,
            column: token.get_src_col() + 1,
        })
    }
}

impl SourceMapper for SourcemapMapper {
    fn map(&self, line: u32, col: u32) -> Option<q_engine::OriginalLocation> {
        self.lookup(line, col)
    }
}

/// A source-map mapper is deliberately optional: production packs do not
/// carry source maps, and diagnostics must not make the success path parse or
/// load a debug sidecar.
pub fn mapper_for(pack: &q_pack::QPack) -> Arc<dyn SourceMapper> {
    match &pack.source_map {
        Some(json) => match SourcemapMapper::parse(json) {
            Ok(m) => Arc::new(m),
            Err(_) => Arc::new(q_engine_quickjs::IdentityMapper),
        },
        None => Arc::new(q_engine_quickjs::IdentityMapper),
    }
}

/// Load an advisory `<pack>.sources.json` sidecar only when tooling requests
/// symbolization. Binding is checked against the exact verified pack bytes;
/// missing or malformed sidecars never affect serving.
pub fn mapper_for_sidecar(
    sidecar_path: &std::path::Path,
    pack_bytes: &[u8],
) -> Result<Arc<dyn SourceMapper>, String> {
    let sidecar =
        q_pack::sources_sidecar::SourcesSidecar::load_and_verify(sidecar_path, pack_bytes)?;
    match sidecar.source_map {
        Some(json) => SourcemapMapper::parse(&json).map(|m| Arc::new(m) as Arc<dyn SourceMapper>),
        None => Ok(Arc::new(q_engine_quickjs::IdentityMapper)),
    }
}
