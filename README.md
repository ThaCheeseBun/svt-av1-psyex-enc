Tiny Rust safe wrapper for SVT-AV1 

[crates.io](https://crates.io/crates/svt-av1-enc)

[docs](https://docs.rs/crate/svt-av1-enc/latest)

# Warning
This crate must be used with SVT-AV1 2.3.0 version. SVT-AV1 library API is subject to change with new releases, so if you want to use it with different version, do it at your own risk!

# Usage
Before using this crate, you have to follow two steps:
1. Choose one of two crate features: `static` or `dynamic`
2. Provide compiled SVT-AV1 library files

The crate has two features:
- `static` - link SVT-AV1 as static library (using `.a` or `.lib` file)
- `dynamic` - link SVT-AV1 as dynamic library (using `.so`, `.dylib` or `.dll` file)

For example:
```toml
svt-av1-enc = { version = "0.1.0", features = ["static"] }
```

Providing library files can be done in two ways:
1. Specify `SVT_AV1_LIB` env variable, which contains path to compiled library files (recommended way)
2. If you not specify `SVT_AV1_LIB` variable, build script will use `pkg-config` (make sure it installed) to find installed in a system SVT-AV1 files

I highly recommend compile SVT-AV1 from source and set `SVT_AV1_LIB` env variable to complied files. Many distros have outdated version of SVT-AV1 and it can cause problems using this crate. Building SVT-AV1 from source is relatively ease and doesn't require a lot of dependencies. Build guide can be found [here](https://gitlab.com/AOMediaCodec/SVT-AV1/-/blob/master/Docs/Build-Guide.md)
