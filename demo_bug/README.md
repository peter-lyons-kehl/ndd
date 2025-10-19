# Fat LTO effect demo

Demo that LLVM can de-duplicate equal `static` literals or variables and `const` even between
crates, that is, affecting 3-rd party ("innocent") code.

Run and see how `lto = "fat"` (Link Time Optimization) affects de-duplication. And, how some types
get de-duplicated even in default `dev` or `release` builds.

- same `static` and `const` values across crates, **without** `lto = "fat"`, even in standard
  `release` build, do **not** get de-duplicated. See [root
  README.md](../README.md#quality-assurance).

The only difference between [`non_lto/`](non_lto) and [`fat_lto/`](fat_lto) is in their
`Cargo.toml`:

- [`fat_lto/Cargo.toml`](fat_lto/Cargo.toml) uses `lto = "fat"`, but
- [`non_lto/Cargo.toml`](non_lto/Cargo.toml) does not.

This requires a filesystem that supports symlinks. [`non_lto/`](non_lto) and [`fat_lto/`](fat_lto)
use symlinks to reuse files from [`../demo_shared_src`](../demo_shared_src) and
[`../demo_shared_scripts`](../demo_shared_scripts).

[`fat_lto/`](fat_lto/) is **different** to `ndd`'s unit tests:

- `ndd` unit tests use [Rust's default `release/dev/MIRI`
  optimization](https://doc.rust-lang.org/nightly/cargo/reference/profiles.html#default-profiles),
  but
- [`fat_lto/`](fat_lto/) enables Fat LTO (in [`fat_lto/Cargo.toml`](fat_lto/Cargo.toml)) not just
  for `release`, **but** for **`dev`, too**.
