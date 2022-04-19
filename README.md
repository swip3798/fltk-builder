# fltk-builder
A crate to enable building an FLTK UI in Rust using a builder pattern. 

## Usage
Just add the following to your project's Cargo.toml file:
```toml
fltk-builder = "^0.1"
```
If you're not interested in the ID map functionality you can it off by disabling the default features: 
```toml
[dependencies]
fltk-builder = { version = "^0.1", default-features = false }
```