use grit_sys;

#[derive(Debug, Clone, Copy)]
pub enum Compression {
    None,
    Lz77,
}

pub struct GritOptions {
    pub bit_depth: u8,
    pub compression: Compression,
}

impl Default for GritOptions {
    fn default() -> Self {
        GritOptions {
            bit_depth: 8,
            compression: Compression::None,
        }
    }
}

// TODO: Add safe wrapper functions around grit_sys bindings here
// Examples:
// - process_tile_sheet(&[u8]) -> Result<Vec<u8>, GritError>
// - generate_tilemap(...) -> Result<Tilemap, GritError>
// - apply_palette(...) -> Result<Vec<u8>, GritError>
