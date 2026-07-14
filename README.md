# GBA Image Editor

A Rust + egui GUI tool for converting images into custom GBA-compatible binary formats using the Grit library.

## Project Structure

```
├── grit-sys/              # Low-level FFI bindings to Grit C++ library
│   ├── Cargo.toml
│   ├── build.rs          # Compiles Grit sources and generates bindings
│   └── src/lib.rs        # Re-exports generated bindings
│
├── gba_image_editor/      # Main GUI application
│   ├── Cargo.toml
│   └── src/
│       ├── main.rs       # App entry point
│       ├── grit.rs       # Safe Rust wrapper around FFI
│       └── ui.rs         # egui interface
│
└── Cargo.toml            # Workspace definition
```

## Next Steps

   - Add file dialogs for image loading
   - Implement preview rendering
   - Add palette management panels
   - Wire up grit function calls to UI buttons

## Dependencies

- **egui**: Immediate-mode GUI framework
- **eframe**: egui windowing backend
- **grit-sys**: Low-level bindings to Grit (this workspace)
- **cc**: C/C++ compilation during build
- **bindgen**: Automatic FFI binding generation

## Bindgen

Summary of the Rust Bindings generated from grit 1.21.1

### Main functions:
- `grit_alloc() -> *mut GritRec` — Allocate GritRec instance
- `grit_free(gr: *mut GritRec)` — Deallocate GritRec
- `grit_init(gr: *mut GritRec)` — Initialize GritRec
- `grit_init_from_dib(gr: *mut GritRec) -> bool` — Initialize from DIB
- `grit_clear(gr: *mut GritRec)` — Clear GritRec
- `grit_copy_options(dst: *mut GritRec, src: *const GritRec)` — Copy options between instances
- `grit_copy_strings(dst: *mut GritRec, src: *const GritRec)` — Copy strings between instances
- `grit_run(gr: *mut GritRec) -> bool` — Execute grit processing
- `grit_validate(gr: *mut GritRec) -> bool` — Validate GritRec state
- `grit_prep(gr: *mut GritRec) -> bool` — Prepare for export
- `grit_export(gr: *mut GritRec) -> bool` — Export processed data
- `grit_compress(dst: *mut RECORD, src: *const RECORD, flags: u32_) -> bool` — Compress RECORD data with flags

### GritRec

Modifying the behaviour is based on modifying the GritRec struct directly:
```rust
pub struct GritRec {
    // Path to source bitmap
    pub srcPath: *mut c_char,
    // Source bitmap
    pub srcDib: *mut CLDIB,
    // Output path directory (-o {name} )
    pub dstPath: *mut c_char,
    // Output symbol name (-s {name} )
    pub symName: *mut c_char,
    // Output file type (-ft{type} )
    pub fileType: u8,
    // Type of extension strings used (-ftb vs -ftB)
    pub extType: u8,
    // Create header file (-fh[!] )
    pub bHeader: bool,
    // Append to existing file (-fa)
    pub bAppend: bool,
    // Global export toggle (?)
    pub bExport: bool,
    // RIFFed data
    pub bRiff: bool,
    // Export rect, left (-al {number} )
    pub areaLeft: c_int,
    // Export rect, top (-at {number} )
    pub areaTop: c_int,
    // Export rect, right (-ar {number} )
    pub areaRight: c_int,
    // Export rect, bottom (-ab {number} )
    pub areaBottom: c_int,
    // Graphics process mode
    pub gfxProcMode: echar,
    // Graphics data type (-gu{num} )
    pub gfxDataType: echar,
    // Graphics compression type
    pub gfxCompression: echar,
    // Graphics mode (tile, bmp, bmpA)
    pub gfxMode: echar,
    // texture operations enabled
    pub texModeEnabled: bool,
    // Output bitdepth (-gB{num} )
    pub gfxBpp: u8,
    // Output texmode, if it is a special texture mode
    pub gfxTexMode: u8,
    // Input image has transparent color
    pub gfxHasAlpha: bool,
    // Transparent color (-gT {num} )
    pub gfxAlphaColor: RGBQUAD,
    // Pixel value offset (-ga {num})
    pub gfxOffset: u32,
    // Graphics are shared (-gS)
    pub gfxIsShared: bool,
    // Map process mode (-m)
    pub mapProcMode: echar,
    // Map data type (-mu {num} )
    pub mapDataType: echar,
    // Map compression type (-mz{char} )
    pub mapCompression: echar,
    // Map tile-reduction mode (-mR[tpf,48a] )
    pub mapRedux: echar,
    // Map layout mode (-mL{char} )
    pub mapLayout: echar,
    // Format describing packed mapsels (GBA Text entries)
    pub msFormat: MapselFormat,
    // Tile width (in pixels) (-tw{num} )
    pub tileWidth: u8,
    // Tile height (in pixels) (-th{num} )
    pub tileHeight: u8,
    // Meta-tile/object width (in tiles) (-Mw{num} )
    pub metaWidth: u8,
    // Meta-tile/object height (in tiles) (-Mh{num} )
    pub metaHeight: u8,
    // Tiles are arranged in column-major order (-tc)
    pub bColMajor: bool,
    // Palette process mode (-p)
    pub palProcMode: echar,
    // Palette data type
    pub palDataType: echar,
    // Palette compression type
    pub palCompression: echar,
    // Has special transparency index
    pub palHasAlpha: bool,
    // Transparent palette entry
    pub palAlphaId: u32,
    // First palette entry to export
    pub palStart: c_int,
    // Final palette entry to export (exclusive)
    pub palEnd: c_int,
    // Whether the user set the palette end
    pub palEndSet: bool,
    // Shared palette (-pS),
    pub palIsShared: bool,
    pub shared: *mut GritShared,
}
```

### Compression:
Explicit compression functions are exposed as well
- `ECprsTag` — values: CPRS_FAKE_TAG, CPRS_LZ77_TAG, CPRS_HUFF_TAG, CPRS_HUFF8_TAG, CPRS_RLE_TAG
- `cprs_create_header(size: uint, tag: u8) -> u32` — Build compression header
- `cprs_compress(dst: *mut RECORD, src: *const RECORD, tag: ECprsTag) -> bool` — Compress based on tag
- `cprs_decompress(dst: *mut RECORD, src: *const RECORD) -> bool` — Decompress generic
- 
All following explicit compression algorithms are `(dst: *mut RECORD, src: *const RECORD) -> uint`
- `fake_compress`, `fake_decompress` — No-op compression
- `lz77gba_compress`, `lz77gba_decompress` — LZ77 codec
- `huffgba_compress` — Huffman codec, no decompression available
- `rle8gba_compress`, `rle8gba_decompress` — Run Length Encoding codec

### File I/O:
- `xp_array_c`, `xp_data_c` — Export arrays/data to a C file, to be compiled into GBA code.
- `xp_array_gas`, `xp_data_gas` — Export arrays/data as gas assembly format
- `im_data_gas` — Import from gas assembly
- `xp_data_bin` — Export to binary file

### Windows Version Constants:
- WINAPI_FAMILY variants
- _WIN32_WINNT versions (NT4–WIN10)
- _WIN32_IE versions (IE2.0–IE11.0)
- NTDDI_* versions (Windows NT2K–WIN10)

### Other:
- `RECORD` struct type — Data container for compression functions
- `path_repl_ext` — Replace file extension
- `str_fix_ident` — Normalize identifiers
- `log_init`, `log_exit`, `log_set_level`, `log_get_level`, `log_set_stream`, `lprintf` — Internal logging
- `__BindgenBitfieldUnit<Storage>` — Handles bitfield packing/unpacking with endianness support (get_bit, set_bit, get, set methods)
- `__IncompleteArrayField<T>` — Represents C arrays of unknown size with slice conversion methods
- `__BindgenUnionField<T>` — Wraps union fields for safe transmutation and access
