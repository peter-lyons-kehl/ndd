# Changelog

This reflects GIT commits on `main` branch (the default branch), that is, `stable` versions
(compatible with `stable` Rust toolchain). Some of the below versions may not have been published to
[crates.io](https://crates.io/crates/ndd) but were skipped.

[`nightly` GIT branch](https://github.com/peter-lyons-kehl/ndd/tree/nightly) may occasionally be
behind `main`. See also [CONTRIBUTING.md](CONTRIBUTING.md).

## 0.2.6 (stable) and 0.3.6-nightly

- Renamed `cross-crate-demo` -> `cross-crate-demo-problem`.
- Moved `cross-crate-demo/bin*/*.sh` scripts one level deeper to `invocation_scripts`.
- API
- `NonDeDuplicatedStr`

## 0.2.5 (stable) and 0.3.5-nightly

- `NonDeDuplicated`'s generic parameter `T` must now implement
  [`core::any::Any`](https://doc.rust-lang.org/nightly/core/any/trait.Any.html).
- Tests with [MIRI](https://github.com/rust-lang/miri).
- [`cross-crate-demo`](cross-crate-demo/) demonstrates effect of Fat LTO (Link Time Optimization)
  across crates. Also invoked from GitHub Actions.
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
