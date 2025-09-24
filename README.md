#  Non-De-Duplicated

## Problem

Rust (or, rather, LLVM) by default de-duplicates or reuses `static` data and its parts. For most
purposes that is good: The result binary is smaller, and because of more successful cache hits the
execution may be faster.

However, that is counter-productive when the code identifies/compares `static` data by reference
(whether a Rust reference/slice, or a pointer/pointer range). For example, an existing Rust/3rd
party API may accept ("ordinary") references/slices. You may want to extend that API's
protocol/behavior with special handling when the client sends in a designated `static` by
reference/slice/pointer/pointer range.

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

`NonDeDuplicated` uses
[`core::cell::Cell`](https://doc.rust-lang.org/nightly/core/cell/struct.Cell.html) to hold the data
passed in by the user. The only access it gives to the inner data is through shared references.
There is no mutation access. (If the inner data allows interior mutability, then it can't implement
[`core::marker::Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html), and then
`NonDeDuplicated` doesn't implement `Send` either and it can't be stored in a `static`.)

Unlike `Cell` (and friends), `NonDeDuplicated` **does** implement
[`core::marker::Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html) (if the inner
data's type implements `Sync`, too). It can safely do so, because it never provides mutable access.
That is similar to how
[`std::sync::Mutex`](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html) implements
`Sync`, too.

See a test [`src/lib.rs` -> `addresses_unique_between_const_and_ndd()`](src/lib.rs).
