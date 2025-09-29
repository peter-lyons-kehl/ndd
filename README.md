# ndd (Non-De-Duplicated)

## Problem

Rust (or, rather, LLVM) by default de-duplicates or reuses `static` data and its parts. For most
purposes that is good: The result binary is smaller, and because of more successful cache hits the
execution may be faster.

However, that is counter-productive when the code identifies/compares `static` data by memory
address of the reference (whether a Rust reference/slice, or a pointer/pointer range). For example,
an existing Rust/3rd party API may accept ("ordinary") references/slices. You may want to extend
that API's protocol/behavior with signalling/special handling when the client sends in your
designated `static` variable by reference/slice/pointer/pointer range. (Your special handler may
cast such references/slices to pointers and compare them by address with
[`core::ptr::eq`](https://doc.rust-lang.org/nightly/core/ptr/fn.eq.html).)

You don't want the client, nor the compiler/LLVM, to reuse/share the memory address of such a
designated `static` for any other ("ordinary") `static` or `const` values/expressions. That does
work out of the box when the client passes a reference/slice defined as `static`: (even with the
default `release` optimizations) each static gets its own memory space. See a test [`src/lib.rs` ->
`addresses_unique_between_statics()`]().

However, it is a problem (in release mode) with ("ordinary") `const` values/expressions that equal
in value to the designated `static`. Rust/LLVM uses one matching `static`'s address for references
to the same value defined as `const`. See a test [`src/lib.rs` ->
`addresses_not_unique_between_const_and_static()`](src/lib.rs). And such `const` definitions could
be in 3rd party (innocent) code!

## Solution

`ndd:NonDeDuplicated` uses
[`core::cell::Cell`](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html) to hold the data
passed in by the user. The only access it gives to the inner data is through shared references.
There is no mutation access.

Unlike `Cell` (and friends), `NonDeDuplicated` **does** implement
[`core::marker::Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html) (if the inner
data's type implements `Sync`, too). It can safely do so, because it never provides mutable access,
and it never mutates the inner data. That is similar to how
[`std::sync::Mutex`](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E)
implements `Sync`, too.

See a test [`src/lib.rs` -> `addresses_unique_between_const_and_ndd()`](src/lib.rs).

## Compatibility

`ndd` doesn't need heap (`alloc`) and it's `no_std`-compatible. It compiles with `stable` Rust.
Optionally, you can get `nightly` functionality.

### Forward compatible

`ndd` is planned to be always below version `1.0`. That allows you to specify it as a dependency
with version `0.*`. Then you get the newest version available for your Rust automatically.

## Future/nightly functionality

We prefer not to introduce temporary cargo features. Removing a feature later is a breaking change.
And we don't want just to make such a feature no-op and let it sit around either.

So, instead:

- any functionality depending on `nightly` features known to be stabilized soon at a certain Rust
  version will have `rust-version` set so. They will be automatically available once that Rust
  version becomes stable. Until then they will be available for `nightly`.
- any functionality depending on `nightly` features with no certain stabilization version will have
  `rust-version` set to FAR in the future (`1.9999.x` for now, where `x` reflects the
  existing/planned `1.x` Rust). You can use it by passing `--ignore-rust-version` to `cargo` (see
  [The Cargo Book >
  rust-vesion](https://doc.rust-lang.org/nightly/cargo/reference/rust-version.html)).

### as_array_of_cells

Hopefully, since Rust version 1.91 (on/around October 30, 2025), `ndd::NonDeDuplicated` will have
function `as_array_of_cells`, similar to Rust's
[`core::cell::Cell::as_array_of_cells`](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html#method.as_array_of_cells)
(which will become stable in 1.91). If you need this functionality earlier, use `nightly` and
`--ignore-rust-version`.

If using `--ignore-rust-version` is risky, we can set up a nightly-compatible GIT branch, and you
could import it following [The Cargo Book > Specifying dependencies from git repositories > Choice
of
commit](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#choice-of-commit).
If that seems difficult, or if you want to publish your crate on crates.io, get in touch.

There does not seem to be a way to specify `--ignore-rust-version` (or an equivalent) in
`Cargo.toml`, `config.toml`, `build.rs` or through environment variables (Cargo Book >
[rust-version](https://doc.rust-lang.org/nightly/cargo/reference/rust-version.html),
[environment-variables](https://doc.rust-lang.org/nightly/cargo/reference/environment-variables.html),
[manifest](https://doc.rust-lang.org/nightly/cargo/reference/manifest.html),
[build-scripts](https://doc.rust-lang.org/nightly/cargo/reference/build-scripts.html) and
[config.toml](https://doc.rust-lang.org/nightly/cargo/reference/config.html)). That may be a GOOD
thing: It requires the top level crate/workspace maintainer to acknowledge that it's non-standard.

### as_slice_of_cells

Similar to `as_array_of_cells`, function `as_slice_of_cells` can be implemented with Rust `1.88`.
However, to simplify versioning, it's planned to go together with `as_array_of_cells`. If you need
it earlier, get in touch.

### const Deref and From

With `nightly` Rust toolchain and use of `--ignore-rust-version` you can get
[core::ops::Deref](https://doc.rust-lang.org/nightly/core/ops/trait.Deref.html) and
[core::convert::From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) implemented as
`const`. As of mid 2025, `const` traits are having high traction in Rust. Hopefully this will be
stable not in years, but sooner.

# Streams and versions

- Odd major versions (`0.1`, `0.3`...) are for `nightly` functionality. They require
  `--ignore-rust-version` (example below).
- Even major versions (`0.2`, `0.4`...) are for `stable` functionality.

Always specify `ndd` with version `0.*`, and you will get the newest available for `stable`.

## Quality

Checked and tested (also with [MIRI](https://github.com/rust-lang/miri)):
- `stable` stream:
  - `cargo clippy`
  - `cargo +stable test`
  - `cargo +stable test --release`
  - `cargo +nightly miri test`
- `nightly` stream:
  - `cargo clippy`
  - `cargo +nightly test  --ignore-rust-version`
  - `cargo +nightly test  --ignore-rust-version --release`
  - `cargo +nightly miri test --ignore-rust-version`

## Use cases

Used by
[`hash-injector::signal`](https://github.com/peter-lyons-kehl/hash-injector/blob/main/lib/src/signal.rs).

## Updates

Please subscribe for low frequency updates at
[#1](https://github.com/peter-lyons-kehl/ndd/issues/2).
