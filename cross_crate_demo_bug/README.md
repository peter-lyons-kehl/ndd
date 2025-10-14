# Fat LTO effect demo

Demo that LLVM can de-duplicate equal `static` literals or variables and `const` even between
crates, that is, affecting 3-rd party ("innocent") code.

Run and see how `lto = "fat"` (Link Time Optimization) affects de-duplication. And, how some types
get de-duplicated even in default `dev` or `release` builds.

- same `static` and `const` values across crates, **without** `lto = "fat"`, even in standard
  `release` build, do **not** get de-duplicated. See [root README.md](../README.md#Quality).

The only difference between [`bin_non_lto/`](bin_non_lto) and [`bin_fat_lto/`](bin_fat_lto) is in
their `Cargo.toml`:

- [`bin_fat_lto/Cargo.toml`](bin_fat_lto/Cargo.toml) uses `lto = "fat"`, but
- [`bin_non_lto/Cargo.toml`](bin_non_lto/Cargo.toml) does not.

This requires a filesystem that supports symlinks. [`bin_non_lto`](bin_non_lto) and
[`bin_fat_lto`](bin_fat_lto) use symlinks to reuse files from
[`cross_crate_shared_src`](cross_crate_shared_src) and
[`cross_crate_shared_scripts`](cross_crate_shared_scripts).

[`bin_fat_lto/`](bin_fat_lto/) is **different** to `ndd`'s unit tests:

- `ndd` unit tests use [Rust's default `release/dev/miri`
  optimization](https://doc.rust-lang.org/nightly/cargo/reference/profiles.html#default-profiles),
  but
- [`bin_fat_lto/`](bin_fat_lto/) enables Fat LTO (in
  [`bin_fat_lto/Cargo.toml`](bin_fat_lto/Cargo.toml)) not just for `release`, **but** for **`dev`,
  too**.
