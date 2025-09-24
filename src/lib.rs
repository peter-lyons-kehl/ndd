#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]

use core::cell::Cell;

#[repr(transparent)]
pub struct NonDeDuplicated<T: ?Sized> {
    cell: Cell<T>,
}

impl<T> NonDeDuplicated<T> {
    pub const fn new(data: T) -> Self {
        Self {
            cell: Cell::new(data),
        }
    }

    pub const fn get(&self) -> &T {
        let ptr = self.cell.as_ptr();
        unsafe { &*ptr }
    }
}

#[deprecated = "function name may change"]
impl<T> NonDeDuplicated<[T]> {
    pub const fn as_slice_of_cells(&self) -> &[NonDeDuplicated<T>] {
        unsafe { core::mem::transmute(self.cell.as_slice_of_cells()) }
    }
}
/* TODO Since Rust 1.92:

impl<T, const N: usize> NonDeDuplicated<[T; N]> {
    pub const fn as_array_of_cells(&self) -> &[NonDeDuplicated<T>; N] {
        unsafe { core::mem::transmute(self.cell.as_array_of_cells()) }
    }
}*/

// @TODO impl Deref, From, Into

// Automatic:
//
//unsafe impl<T: ?Sized + Send> Send for NonDeDuplicated<T> {}

/// For now, `Sync` requires that `T` is both `Sync` AND `Send`, following
/// [std::sync::Mutex](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E).
/// However, from https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html it seems that `T:
/// Send` may be unnecessary? Please advise.
unsafe impl<T: ?Sized + Send + Sync> Sync for NonDeDuplicated<T> {}

#[cfg(test)]
mod tests {
    use super::*;

    type U8Array = [u8; 3];
    const ARR_CONST: U8Array = [b'A', b'B', b'C'];
    static ARR_STATIC_1: U8Array = [b'A', b'B', b'C'];
    static ARR_STATIC_2: U8Array = [b'A', b'B', b'C'];

    #[test]
    fn addresses_unique_between_statics() {
        assert!(!core::ptr::eq(
            (&ARR_STATIC_1).as_ptr(),
            (&ARR_STATIC_2).as_ptr()
        ));
    }

    #[cfg(not(debug_assertions))]
    /// In release, [ARR_CONST] gets optimized away and points to the same address as [ARR_STATIC_1]!
    #[should_panic(
        expected = "assertion failed: !core::ptr::eq((&ARR_STATIC_1).as_ptr(), (&ARR_CONST).as_ptr())"
    )]
    #[test]
    fn addresses_not_unique_between_const_and_static() {
        assert!(!core::ptr::eq(
            (&ARR_STATIC_1).as_ptr(),
            (&ARR_CONST).as_ptr()
        ));
    }

    static ARR_NDD: NonDeDuplicated<U8Array> = NonDeDuplicated::new(ARR_CONST);
    static ARR_NDD_REF: &'static U8Array = ARR_NDD.get();
    #[test]
    fn addresses_unique_between_const_and_ndd() {
        assert!(!core::ptr::eq(ARR_NDD_REF.as_ptr(), (&ARR_CONST).as_ptr()));
    }
}
