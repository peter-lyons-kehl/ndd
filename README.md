# ndd (Non-De-Duplicated)

## Summary

Zero-cost transparent wrapper. For `static` variables guaranteed not to share memory with any other
`static` or `const` (or local literals). Especially for `static` data (single
variables/arrays/slices) referenced with references/slices/pointers that are **compared by
address**.

## Problem

Rust (or, rather, LLVM) by default de-duplicates or reuses **addresses** of `static` variables in
`release` builds. And somewhat in `debug` builds, too. For most purposes that is good: The result
binary is smaller, and because of more successful cache hits, the execution may be faster.

However, that is counter-productive when the code identifies/compares `static` data by memory
address of the reference (whether a Rust reference/slice, or a pointer/pointer range). For example,
an existing Rust/3rd party API may accept ("ordinary") references/slices. You may want to extend
that API's protocol/behavior with signalling/special handling when the client sends in your
designated `static` variable by reference/slice/pointer/pointer range. (Your special handler may
cast such references/slices to pointers and compare them by address with
[`core::ptr::eq()`](https://doc.rust-lang.org/nightly/core/ptr/fn.eq.html).)

Then you don't want the client, nor the compiler/LLVM, to reuse/share the memory address of such a
designated `static` for any other ("ordinary") `static` or `const` values/expressions, or local
numerical/character/byte/string slice literals. That does work out of the box when the client passes
a reference/slice defined as `static`: (even with the default `release` optimizations) each `static`
gets its own memory space. See a test [`src/lib.rs` ->
`addresses_unique_between_statics()`](https://github.com/peter-lyons-kehl/ndd/blob/26d743d9b7bbaf41155e00174f8827efca5d5f32/src/lib.rs#L72).

However, there is a problem (in `release` mode, and for some types even in `debug` mode). It affects
("ordinary") `const` values/expressions that equal in value to any `static` (which may be your
designated `static`). Rust/LLVM re-uses address of one such matching `static`' for references to
equal value(s) defined as `const`. See a test [`src/lib.rs` ->
`addresses_not_unique_between_const_and_static()`](https://github.com/peter-lyons-kehl/ndd/blob/26d743d9b7bbaf41155e00174f8827efca5d5f32/src/lib.rs#L95).
Such `const`, `static` or literal could be in 3rd party code, and even private (see
[`demo-fat-lto`](./demo-fat-lto))!

Things get worse: `debug` builds don't have this consistent.

- For some types (`u8`, numeric primitive-based enums) `debug` builds don't reuse `static` addresses
  for references/slices to `const` values. But
- For other types (`str`), `debug` builds do reuse them...
- Only `MIRI` doesn't reuse `static` addresses at all.

Even worse so: `release` builds don't have this consistent.

## Solution

`ndd:NonDeDuplicated` uses
[`core::cell::Cell`](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html) to hold the data
passed in by the user. There is no mutation and no mutation access. The only access it gives to the
inner data is through shared references.

Unlike `Cell` (and friends), `NonDeDuplicated` **does** implement
[`core::marker::Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html) (if the inner
data's type implements `Send` and  `Sync`). It can safely do so, because it never provides mutable
access, and it never mutates the inner data. That is similar to how
[`std::sync::Mutex`](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E)
implements `Sync`, too.

See a test [`src/lib.rs` ->
`addresses_unique_between_const_and_ndd()`](https://github.com/peter-lyons-kehl/ndd/blob/26d743d9b7bbaf41155e00174f8827efca5d5f32/src/lib.rs#L102).

## Use

Use `ndd::NonDeDuplicated` to wrap your static data. Use it for (immutable) `static` variables only.
Do **not** use it for locals or on heap. That is validated by implementation of
[core::ops::Drop](https://doc.rust-lang.org/nightly/core/ops/trait.Drop.html), which `panic`-s in
debug builds.

See unit tests in [src/lib.rs](src/lib.rs).

## Compatibility

`ndd` is `no_std`-compatible and it doesn't need heap (`alloc`) either. Release versions
(**even**-numbered major versions, and **not** `-nightly` pre-releases) compile with `stable` Rust.
(More below.)

### Stable is always forward compatible

`ndd` is planned to be always below version `1.0`. (If a need arises for big incompatible
functionality, that can go in a new crate.)

That allows you to specify `ndd` as a dependency with version `0.*`, which will match ANY **major**
versions (below `1.0`, of course). That will match the newest (**even**-numbered major) stable
version (available for your Rust) automatically.

This is special only to `0.*` - it is **not** possible to have a wildcard matching various **major**
versions `1.0` or higher.

### Versioning convention:

- **Even**-numbered major versions (`0.2`, `0.4`...)
  - are for **stable** functionality only.
  - don't use any pre-release identifier (so, nothing like `0.4-alpha`).
  - here they are called **"stable"**, but the version name/identifier doesn't include "stable"
    word.
- **Odd**-numbered major versions (`0.3`, `0.5`...)
  - always contain `-nightly` (pre-release identifier) in their name.
  - are, indeed, for `nightly` (**unstable**) functionality, and need `nightly` Rust toolchain
    (indicated with `rust-toolchain.toml` which is present on `nightly` branch only).
  - include functionality already present in some lower stable versions. Not all of them - only:
    - stable versions with a **lower major** numeric version, and
    - if the stable **major** version is lower by **`0.1` only** (and not by more), then the stable
      **minor** version has to be the **same or lower** (than minor version of the **odd-numbered**
      (`-nightly`)).

    So if `x < z`
    - `0.x.y` (stable)
    - `0.z.y-nightly` is (indeed) `nightly`
      - `0.z.y-nightly` includes all functionality already present in `0.x.y` (stable).
    - But, if `x + 0.1 == z` and `y < w`
      - `0.z.y-nightly` does **not** include any functionality **new** in `0.x.w` (stable), because
        it was **not** present in `0.x.y` yet).
    
    Examples:
    - `0.2.1` (stable)
    - `0.3.1-nightly`
      - `0.3.1-nightly` includes functionality present in `0.2.1` (stable).
    - `0.2.2` (stable)
    - `0.3.2-nightly`
      - `0.3.2-nightly` includes functionality present in `0.2.2` (if they get published), **BUT:**
    - `0.2.1` (stable)
    - `0.3.1-nightly`
      - `0.3.1-nightly` will **not** include functionality present in `0.2.2` that was not present
        in `0.2.1`.
- If needed and if practical, new major versions will use [the SemVer
  trick](https://github.com/dtolnay/semver-trick). See also [The Cargo Book > Dependency
  Resolution](https://rustwiki.org/en/cargo/reference/resolver.html#version-incompatibility-hazards).
  
  However, the only type exported from `ndd` is `ndd::NonDeDuplicated`. It is a zero-cost wrapper
  suitable for immutable `static` variables. It is normally not being passed around as a
  parameter/return type or a composite type. And its functions can get inlined/optimized away. So,
  there shouldn't be any big binary size/speed difference, or usability difference, if there happen
  to be multiple major versions of `ndd` in use at the same time. They would be all isolated. So
  SemVer trick may be unnecessary.

#### Rule of thumb for stable versions

On `stable` Rust, always specify `ndd` with version `0.*`. Then, automatically:

- you will get the newest available even-numbered major (stable) version, and
- your libraries will work with any newer **odd-numbered** major (`-nightly`) version of `ndd`, too,
  if any dependency (direct or transitive) requires it.

#### Rule of thumb for unstable versions

To find out the highest **even-numbered** (stable) version whose functionality is included in a
given **odd-numbered** (`-nightly`) version, decrement the **odd-numbered** version by `0.1` (and
remove the `-nightly` suffix).

### Nightly versioning

We prefer not to introduce temporary cargo features. Removing a feature later is a breaking change.
And we don't want just to make such a feature no-op and let it sit around.

So, instead, any `nightly`-only functionality is in separate version stream(s) that always

- are **pre-releases** (as per [The Cargo Book > Specifying Dependencies >
  Pre-releases](https://doc.rust-lang.org/nightly/cargo/reference/specifying-dependencies.html#pre-releases)
  and [The Cargo Book > The Manifest Format > The version
  field](https://doc.rust-lang.org/nightly/cargo/reference/manifest.html#the-version-field))
  containing `-nightly` in their name.
- use **odd**-numbered major version numbers (`0.3.x`, `0.5.x`...). And, because they are always
  **pre-releases**, their version has to be specified including the pre-release identifier
  `-nightly`. So, **unlike** even-numbered major (stable) versions, `-nightly` versions **cannot**
  be matched with `0.*`. Therefore they will **not** match/auto-update to any other **major**
  version (whether odd or even).

As per Rust resolver rules, a stable (**non**-pre-release) version will NOT match/auto-update to a
**pre-release** version on its own. Therefore, if your crate and/or its dependencies specify `ndd`
version as `0.*`, they will **not** accidentally request an **odd**-numbered major (`-nightly`) on
their own.

They can get a (`-nightly`) version, but only if another crate requires it. That's up to the
consumer.

If you want more control over stable versions, you can fix the **even**-numbered major version, and
use an asterisk mask for the minor version, like `0.2.*`. But then you lose automatic major updates.

### Nightly functionality

Functionality of odd-numbered major (`-nightly`) versions is always subject to change.

The following extra functionality is available on `0.3.1-nightly`:

#### as_array_of_cells

`ndd::NonDeDuplicated` has function `as_array_of_cells`, similar to Rust's
[`core::cell::Cell::as_array_of_cells`](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html#method.as_array_of_cells)
(which will, hopefully, become stable in 1.91).

#### as_slice_of_cells

Similar to `as_array_of_cells`, `ndd::NonDeDuplicated` has function `as_slice_of_cells`. That
**can** be stable with with Rust `1.88`+. However, to simplify versioning, it's bundled in
`-nightly` together with `as_array_of_cells`. If you need it earlier, get in touch.

#### const Deref and From

With `nightly` Rust toolchain and use of `--ignore-rust-version` you can get
[core::ops::Deref](https://doc.rust-lang.org/nightly/core/ops/trait.Deref.html) and
[core::convert::From](https://doc.rust-lang.org/nightly/core/convert/trait.From.html) implemented as
`const`. As of mid 2025, `const` traits are having high traction in Rust. Hopefully this will be
stable not in years, but sooner.

## Quality

Checked and tested (also with [MIRI](https://github.com/rust-lang/miri)):
- `cargo clippy`
- `cargo test`
- `cargo test --release`
- `cargo +nightly miri test`
- `release`-only tests:
  - `demo-fat-lto/static_option_u8.sh`
  - `demo-fat-lto/static_str.sh`
  - `demo-fat-lto/literal_str.sh`

The versioning convention is validated:

- locally with GIT [pre-commit](./pre-commit) hook, and
- with a [GitHub action](.github/workflows/main.yml).

## Use cases

Used by
[`hash-injector::signal`](https://github.com/peter-lyons-kehl/hash-injector/blob/main/lib/src/signal.rs).

## Updates

Please subscribe for low frequency updates at
[#2](https://github.com/peter-lyons-kehl/ndd/issues/2).

## Side fruit

This is `std`-only, but related: `std::sync::mutex::data_ptr(&self)` is now `const` function: pull
request [rust-lang/rust#146904](https://github.com/rust-lang/rust/pull/146904).
