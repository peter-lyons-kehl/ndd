# Changelog

This reflects GIT commits on `main` branch (the default branch), that is, `stable` versions
(compatible with `stable` Rust toolchain). Some of the below versions may not have been published to
[crates.io](https://crates.io/crates/ndd) but were skipped.

[`nightly` GIT branch](https://github.com/peter-lyons-kehl/ndd/tree/nightly) may occasionally be
behind `main`. See also [CONTRIBUTING.md](CONTRIBUTING.md).

<!--
## 0.3.7-nightly

`ndd::infer::NonDeDuplicatedStr` and `ndd::infer::NonDeDuplicatedCStr`
-->

## 0.2.10 (stable)

Renamed test files. More links in documentation.

## 0.2.9 (stable)

`README.md` links to work on both GitHub and `rustdoc` or `crates.io` (more).

## 0.2.8 (stable)

`README.md` links to work on both GitHub and `rustdoc` or `crates.io`.

## 0.2.7 (stable)

- `NonDeDuplicatedCStr` for FFI
  [`core::ffi::CStr`](https://doc.rust-lang.org/nightly/core/ffi/struct.CStr.html)
- Renamed `cross-crate-demo-problem/` to `cross_crate_demo_bug/` (later renamed to `demo_bug/`).
- `cross_crate_shared_scripts/` (later renamed to `demo_shared_scripts/`)
- Created `cross_crate_demo_fix/` (later renamed to `demo_fix/`), added to `README.md` and GitHub
  actions.

## 0.2.6 (stable)

- Renamed `cross-crate-demo` -> `cross-crate-demo-problem`.
- Moved `cross-crate-demo/bin*/*.sh` scripts one level deeper to `invocation_scripts` (later moved
  to `demo_shared_scripts/`).
- API
- `NonDeDuplicatedStr`

## 0.2.5 (stable) and 0.3.5-nightly

- `NonDeDuplicated`'s generic parameter `T` must now implement
  [`core::any::Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html).
- Tests with [MIRI](https://github.com/rust-lang/miri).
- `cross-crate-demo/` (later renamed to `demo_bug/`) demonstrates effect of Fat LTO
  (Link Time Optimization) across crates. Also invoked from GitHub Actions.
- Docs.

## 0.2.4 (stable)

- GitHub Actions on Alpine Linux
- Docs; GH Actions badge in README
- `fat lto` demo
- [`precommit`](./pre-commit) GIT check

## 0.2.3 (stable) and 0.3.3-nightly

- Validation: For `static` variables only. Dropping `ndd::NonDeDuplicated` in debug build panics.
- New tests.
- Documentation: `nightly`, quality,
- Versioning scheme validation: GitHub Actions.
- Versioning scheme validation: local GIT pre-commit hook.

## 0.2.2 (stable)

Documentation.

## 0.2.1 (stable)

Documentation.

## 0.2.0 (stable)

Initial functionality.

## 0.0.0

Crate name registered, but no functionality.
