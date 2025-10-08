# Fat LTO effect demo

Demo that LLVM can de-duplicate equal `static` literals or variables and `const` even between
crates, that is, affecting 3-rd party ("innocent") code.

Run and see how `lto = "fat"` (Link Time Optimization) affects de-duplication:

- same `static` and `const` values across crates, **without** `lto = "fat"`, even in standard
  `release` build, do **not** get de-duplicated:
  - [`bin/literal_str.sh`](bin/literal_str.sh)
  - [`bin/static_option_u8.sh`](bin/static_option_u8.sh)
  - [`bin/static_str.sh`](bin/static_str.sh)
- but, same `static` and `const` values across crates, **with** `lto = "fat"`, **do** get
  de-duplicated:
  - [`bin-fat-lto/literal_str.sh`](bin-fat-lto/literal_str.sh)
  - [`bin-fat-lto/static_option_u8.sh`](bin-fat-lto/static_option_u8.sh)
  - [`bin-fat-lto/static_str.sh`](bin-fat-lto/static_str.sh)

The only difference between [`bin/`](bin) and [`bin-fat-lto/`](bin-fat-lto) is in their
`Cargo.toml`:

- [`bin-fat-lto/Cargo.toml`](bin-fat-lto/Cargo.toml) uses `lto = "fat"`, but
- [`bin/Cargo.toml`](bin/Cargo.toml) does not.

This requires a filesystem that supports symlinks. ([`bin`](bin) and [`bin-fat-lto`](bin-fat-lto)
use symlinks to reuse files from [`bin-shared`](bin-shared).
