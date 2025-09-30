# ndd (Non-De-Duplicated)

## Summary

Zero-cost transparent wrapper. Use when comparing `static` references/slices/pointers by address.
For `static` variables guaranteed not to share memory with any other `static` or `const`.

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
[`core::ptr::eq()`](https://doc.rust-lang.org/nightly/core/ptr/fn.eq.html).)

You don't want the client, nor the compiler/LLVM, to reuse/share the memory address of such a
designated `static` for any other ("ordinary") `static` or `const` values/expressions. That does
work out of the box when the client passes a reference/slice defined as `static`: (even with the
default `release` optimizations) each static gets its own memory space. See a test [`src/lib.rs` ->
`addresses_unique_between_statics()`](https://github.com/peter-lyons-kehl/ndd/blob/26d743d9b7bbaf41155e00174f8827efca5d5f32/src/lib.rs#L72).

However, it is a problem (in release mode) with ("ordinary") `const` values/expressions that equal
in value to the designated `static`. Rust/LLVM uses one matching `static`'s address for references
to equal value(s) defined as `const`. See a test [`src/lib.rs` ->
`addresses_not_unique_between_const_and_static()`](https://github.com/peter-lyons-kehl/ndd/blob/26d743d9b7bbaf41155e00174f8827efca5d5f32/src/lib.rs#L95).
And such `const` definitions could even be in 3rd party (innocent) code!

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

## Compatibility

`ndd` is `no_std`-compatible and it doesn't need heap (`alloc`) either. Release versions
(**even**-numbered major versions, and **not** `-nightly` pre-releases) compile with `stable` Rust.
(More below).

### Stable is always forward compatible

`ndd` is planned to be always below version `1.0`. That allows you to specify it as a dependency
with version `0.*` (which is **not** possible for `1.0` and higher). That will match the newest
(**even**-numbered major) stable version (available for your Rust) automatically.

### Stable and nightly

Versioning convention:

- **Even**-numbered major versions (`0.2`, `0.4`...)
  - are for `stable` functionality only.
  - don't use any pre-release identifier (so, nothing like `0.4-alpha`).
- **Odd**-numbered major versions (`0.3`, `0.5`...)
  - are, indeed, for `nightly` (unstable) functionality, and need `nightly` Rust toolchain.
  - always contain `-nightly` (pre-release identifier) in their name.
  - include functionality already present in lower stable versions with the numeric version lower by
    (major) `1`.

    So if `z = x + 1` then
    - `0.x.y` (stable) and
    - `0.z.y-nightly`
    
    then `0.z.y-nightly` includes all functionality already present in `0.x.y` (stable).
    
    Examples:
    - `0.2.1` (stable) and
    - `0.3.1-nightly`
      - `0.3.1-nightly` includes functionality present in `0.2.1` (stable).
    - `0.2.2` (stable) and
    - `0.3.2-nightly`
      - `0.3.2-nightly` includes functionality present in `0.2.2` (if they get published), **BUT:**
    - `0.2.1` (stable) and
    - `0.3.1-nightly`
      - `0.3.1-nightly` will **not** include functionality present in `0.2.2` that was not present
        in `0.2.1`.
- If needed and if practical, new major versions will use [the SemVer
  trick](https://github.com/dtolnay/semver-trick). See also [The Cargo Book > Dependency
  Resolution](https://rustwiki.org/en/cargo/reference/resolver.html#version-incompatibility-hazards).
  
  However, `ndd`'s only exported type is `ndd::NonDeDuplicated`. It is a zero-cost wrapper suitable
  for immutable `static` variables. It is **not** intended for function parameters, local variables
  or as a composite type (if you do have such a use case, please get in touch).
  
  Since `ndd::NonDeDuplicated` is not being passed around, and its functions can get
  inlined/optimized away, there shouldn't be any big binary size/speed or usability difference if
  there happen to be multiple major versions of `ndd` in use at the same time. So SemVer trick may
  be unnecessary.

Rule of thumb: On `stable` Rust, always specify `ndd` with version `0.*`. Then, automatically

- you will get the newest available even-numbered major (`stable`) version, and
- your libraries will work with any newer odd-numbered major (`-nightly`) version of `ndd`, too, if
  any dependency (direct or transitive) requires it.

### Nightly

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
  `-nightly`. So, **unlike** odd-numbered major (`stable`) versions, `-nightly` versions they
  **cannot** be matched with `0.*`. Therefore they
  - will **not** match/auto-update to even-numbered major (`stable`) versions, and
  - will **not** match/auto-update to higher major versions (whether odd or even) either.
  
So a `stable` (**non**-pre-release) version will NOT match/auto-update to a **pre-release** version
on its own. Therefore, if your crate and its dependencies specify `ndd` version as `0.*`, they will
**not** accidentally request **odd**-numbered major (`-nightly`) on their own. They would get
(`-nightly`) version only if another crate requires it - but that's up to the consumer.

If you want more control over `stable` versions, you can specify the **even**-numbered version mask,
like `0.2.*`. But then you lose automatic major updates.

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

## Use cases

Used by
[`hash-injector::signal`](https://github.com/peter-lyons-kehl/hash-injector/blob/main/lib/src/signal.rs).

## Updates

Please subscribe for low frequency updates at
[#2](https://github.com/peter-lyons-kehl/ndd/issues/2).
