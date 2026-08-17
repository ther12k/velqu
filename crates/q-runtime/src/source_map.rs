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

pub fn mapper_for(pack: &q_pack::QPack) -> Arc<dyn SourceMapper> {
    match &pack.source_map {
        Some(json) => match SourcemapMapper::parse(json) {
            Ok(m) => Arc::new(m),
            Err(_) => Arc::new(q_engine_quickjs::IdentityMapper),
        },
        None => Arc::new(q_engine_quickjs::IdentityMapper),
    }
}
