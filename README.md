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
