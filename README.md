#  ndd (Non-De-Duplicated)

## Problem

Rust (or, rather, LLVM) by default de-duplicates or reuses `static` data and its parts. For most
purposes that is good: The result binary is smaller, and because of more successful cache hits the
execution may be faster.

However, that is counter-productive when the code identifies/compares `static` data by reference
(whether a Rust reference/slice, or a pointer/pointer range). For example, an existing Rust/3rd
party API may accept ("ordinary") references/slices. You may want to extend that API's
protocol/behavior with signalling/special handling when the client sends in a designated `static` by
reference/slice/pointer/pointer range. Your special handler may cast such references/slices to
pointers and compare them by address with
[`core::ptr::eq`](https://doc.rust-lang.org/nightly/core/ptr/fn.eq.html).

But you don't want the client, nor the compiler/LLVM, to reuse/share the memory address of such a
designated `static` for any other ("ordinary") `static` or `const` values/expressions. That does
work out of the box when the client passes a reference/slice defined as `static`: It seems that
(with the default `release` optimizations) each static gets its own memory space. See a test
[`src/lib.rs` -> `addresses_unique_between_statics()`](src/lib.rs).

However, it is a problem (in release mode) with ("ordinary") `const` values/expressions that equal
in value to the designated `static`. Rust/LLVM uses the `static` address for references to the same
value defined as `const`. See a test [`src/lib.rs` ->
`addresses_not_unique_between_const_and_static()`](src/lib.rs).

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

`ndd` doesn't need heap (`alloc`) and it's `no_std`-compatible. It compiles with `stable` Rust, unless you need `const_deref` feature.

### as_array_of_cells

Since Rust version 1.92 (on/around December 11, 2025), `ndd::NonDeDuplicated` will have function
`as_array_of_cells`, similar to Rust's `core::cell::Cell::as_array_of_cells` (which will become
stable in 1.92). If you need this earlier, get in touch. (We prefer not to introduce a temporary
cargo feature for this. Removing a feature later is a breaking change. And we don't want just to
make such a feature no-op and let it sit around either.)

## Quality

Tested with:

- `cargo +stable test`, and
- using [MIRI](https://github.com/rust-lang/miri):
  - `cargo +nightly miri test`
  - `cargo +nightly miri test  --features=const_convert`.

## Use cases

Used by
[`hash-injector::signal`](https://github.com/peter-lyons-kehl/hash-injector/blob/main/lib/src/signal.rs).
