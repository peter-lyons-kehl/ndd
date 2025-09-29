#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![feature(const_convert, const_trait_impl)]

use core::cell::Cell;
use core::ops::Deref;

/// A zero-cost  wrapper guaranteed not to share its memory location with any other valid (in-scope)
/// variable (even `const` equal to the inner value). Use for `static` variables that have their
/// addresses compared with [core::ptr::eq].
///
/// It has same size, layout and alignment as type parameter `T`.
#[repr(transparent)]
pub struct NonDeDuplicated<T: ?Sized> {
    cell: Cell<T>,
}

impl<T> NonDeDuplicated<T> {
    /// Construct a new instance.
    pub const fn new(value: T) -> Self {
        Self {
            //Using core::hint::black_box() seems unnecessary.
            //cell: Cell::new(core::hint::black_box(data)),
            cell: Cell::new(value),
        }
    }

    /// Get a reference.
    pub const fn get(&self) -> &T {
        let ptr = self.cell.as_ptr();
        unsafe { &*ptr }
    }
}

impl<T> const Deref for NonDeDuplicated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T> const From<T> for NonDeDuplicated<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> NonDeDuplicated<[T]> {
    pub const fn as_slice_of_cells(&self) -> &[NonDeDuplicated<T>] {
        unsafe { core::mem::transmute(self.cell.as_slice_of_cells()) }
    }
}

impl<T, const N: usize> NonDeDuplicated<[T; N]> {
    pub const fn as_array_of_cells(&self) -> &[NonDeDuplicated<T>; N] {
        unsafe { core::mem::transmute(self.cell.as_array_of_cells()) }
    }
}

/// For now, [Sync] requires that `T` is both [Sync] AND [Send], following
/// [std::sync::Mutex](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E).
/// However, from https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html it seems that `T:
/// Send` may be unnecessary? Please advise.
///
/// Either way, [NonDeDuplicated] exists specifically for static variables. Those get never moved
/// out. So, unlike [std::sync::Mutex], [NonDeDuplicated] itself doesn't need to implement [Send].
unsafe impl<T: ?Sized + Send + Sync> Sync for NonDeDuplicated<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    const fn expect_sync_ref<T: Sync>() {}
    const _: () = expect_sync_ref::<NonDeDuplicated<u8>>();

    type A = u8;
    const A_CONST: A = b'A';
    static A_STATIC_1: A = b'A';
    static A_STATIC_2: A = b'A';

    #[test]
    fn addresses_unique_between_statics() {
        assert!(!ptr::eq(&A_STATIC_1, &A_STATIC_2));
    }

    fn _deref() -> &'static u8 {
        static N: NonDeDuplicated<u8> = NonDeDuplicated::<u8>::new(0);
        &*N
    }

    #[test]
    fn deref_of_copy_type() {
        static N: NonDeDuplicated<u8> = NonDeDuplicated::<u8>::new(0);

        let deref = &*N;
        let get = N.get();
        assert!(ptr::eq(deref, get));
    }

    #[cfg(not(any(debug_assertions, miri)))]
    /// In release, [ARR_CONST] gets optimized away and points to the same address as
    /// [ARR_STATIC_1]!
    #[should_panic(expected = "assertion failed: !ptr::eq(&A_STATIC_1, &A_CONST)")]
    #[test]
    fn addresses_not_unique_between_const_and_static() {
        assert!(!ptr::eq(&A_STATIC_1, &A_CONST));
    }

    static A_NDD: NonDeDuplicated<A> = NonDeDuplicated::new(A_CONST);
    static A_NDD_REF: &'static A = A_NDD.get();
    #[test]
    fn addresses_unique_between_const_and_ndd() {
        assert!(!ptr::eq(A_NDD_REF, &A_CONST));
        assert!(!ptr::eq(A_NDD_REF, &A_STATIC_1));
        assert!(!ptr::eq(A_NDD_REF, &A_STATIC_2));
    }
}
