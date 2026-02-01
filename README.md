# liblas

Parse .las Files in Rust

[Canadian Well Logging Society LAS File 2.0 Specification](https://www.cwls.org/wp-content/uploads/2017/02/Las2_Update_Feb2017.pdf)

# Installation

**To use programmatically**

```bash
cargo add liblas
```

**To use CLI globally**

```bash
cargo install liblas
```

# Usage

```rust
let my_las_file = LasFile::parse("/some/file.las".into())?;
// To json string?
let json_str = my_las_file.to_json_str()?;
```

# Examples

Updated examples coming soon!

