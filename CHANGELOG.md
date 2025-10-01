# Changelog

## 0.3.3-nightly

Removed `ndd::NonDeDuplicated::as_slice_of_cells`. It required `ndd::NonDeDuplicated`'s generic
param `T` to be `?Sized`, but that is incompatible with `Drop` trait. Anyway, storing variable-sized
slices in top-level statics is not possible.

Even though `ndd::NonDeDuplicated` implements `Deref`, and it **does** give access to the inner data
through a reference, it is **not** a fat pointer (to variable/dynamically-sized data).

## 0.2.3 (and included in 0.3.3-nightly)

- Validation: For `static` variables only. So `Drop::drop(...)` of `ndd::NonDeDuplicated` in debug
  build panics.
- New tests.
- Documentation: `nightly`, quality,
- Versioning scheme validation: GitHub Actions.
- Versioning scheme validation: local GIT pre-commit hook.

## 0.3.1-nightly

- `const` impl of `Deref` and `From`.
- `as_slice_of_cells`
- `as_array_of_cells`

## 0.2.2

Documentation.

## 0.2.1

Documentation.

## 0.2.0

Initial functionality.

## 0.0.0

Crate name registered, but no functionality.
