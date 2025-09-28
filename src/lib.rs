#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(feature = "const_convert", feature(const_convert, const_trait_impl))]

use core::cell::Cell;
use core::hint;
use core::ops::Deref;

#[repr(transparent)]
pub struct NonDeDuplicated<T: ?Sized> {
    cell: Cell<T>,
}

impl<T> NonDeDuplicated<T> {
    pub const fn new(data: T) -> Self {
        Self {
            cell: Cell::new(hint::black_box(data)),
        }
    }

    pub const fn get(&self) -> &T {
        let ptr = self.cell.as_ptr();
        unsafe { &*ptr }
    }
}

impl<T> NonDeDuplicated<[T]> {
    pub const fn as_slice_of_cells(&self) -> &[NonDeDuplicated<T>] {
        unsafe { core::mem::transmute(self.cell.as_slice_of_cells()) }
    }
}
/* TODO Since Rust 1.92:

impl<T, const N: usize> NonDeDuplicated<[T; N]> {
    pub const fn as_array_of_cells(&self) -> &[NonDeDuplicated<T>; N] {
        unsafe { mem::transmute(self.cell.as_array_of_cells()) }
    }
}*/

#[cfg(feature = "const_convert")]
macro_rules! impl_const_trait {
    ($trait_name:ty, $( $body:item )+) => {
        impl<T> const $trait_name for $crate::NonDeDuplicated<T> {
            $( $body )+
        }
    };
}
#[cfg(not(feature = "const_convert"))]
macro_rules! impl_const_trait {
    ($trait_name:ty, $( $body:item )+) => {
        impl<T> $trait_name for $crate::NonDeDuplicated<T> {
            $( $body )+
        }
    };
}

impl_const_trait! {Deref,
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl_const_trait! {From<T>,
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

// Automatic:
//
//unsafe impl<T: ?Sized + Send> Send for NonDeDuplicated<T> {}

/// For now, `Sync` requires that `T` is both `Sync` AND `Send`, following
/// [std::sync::Mutex](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E).
/// However, from https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html it seems that `T:
/// Send` may be unnecessary? Please advise.
///
/// Either way, NonDeDuplicated exists specifically for static "variables". Those get never moved
/// out. So, unlike `std::sync::Mutex`, NonDeDuplicated doesn't need to implement `Send`.
unsafe impl<T: ?Sized + Send + Sync> Sync for NonDeDuplicated<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    const fn expect_sync_ref<T: Sync>() {}
    const fn expect_send_ref<T: Send>() {}
    const _: () = expect_send_ref::<NonDeDuplicated<u8>>();
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

    #[cfg(not(debug_assertions))]
    /// In release, [ARR_CONST] gets optimized away and points to the same address as
    /// [ARR_STATIC_1]!
    #[should_panic(expected = "assertion failed: !ptr::eq(&ARR_STATIC_1, &ARR_CONST)")]
    #[test]
    fn addresses_not_unique_between_const_and_static() {
        assert!(!ptr::eq(&ARR_STATIC_1, &ARR_CONST));
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
