#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test)), no_std)]
#![feature(const_convert, const_trait_impl)]

use core::any::Any;
use core::cell::Cell;
use core::ops::Deref;

/// A zero-cost  wrapper guaranteed not to share its memory location with any other valid (in-scope)
/// variable (even `const` equal to the inner value). Use for `static` variables that have their
/// addresses compared with [core::ptr::eq].
///
/// It has same size, layout and alignment as type parameter `T`.
///
/// `T` must implement [Any]. That is automatic for any types that don't have any lifetimes (other
/// than `'static`). This requirement givesn an earlier error when `NonDeDuplicated` is used other
/// than intended.
///
/// We don't just limit to be `:'static`, because then `NonDeDuplicated` could still be somewhat
/// used with non-static lifetimes, and the error would surface only later. The following **would**
/// compile:
/// ```rust
/// # use core::cell::Cell;
///
/// pub struct NonDeDuplicatedStatic<T: 'static> {
///    cell: Cell<T>,
///}
///
/// type NddU8ref<'a> = NonDeDuplicatedStatic<&'a u8>;
///
/// fn callee<'a>(r: NddU8ref<'a>) {}
/// ```
/// Only the following would then fail (to compile):
/// ```rust,ignore
/// fn caller<'a>(r: &'a u8) { let u = 0u8; callee(NonDeDuplicatedStatic::new(uref)); }
/// ```
///
/// But, by requiring `NonDeDuplicated`'s generic parameter `T` to implement [Any] the first
/// example above fails, too. That prevents mistakes earlier.
#[repr(transparent)]
pub struct NonDeDuplicated<T: Any> {
    cell: Cell<T>,
}

impl<T: Any> NonDeDuplicated<T> {
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

impl<T: Any> const Deref for NonDeDuplicated<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.get()
    }
}

impl<T: Any> const From<T> for NonDeDuplicated<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T: Any, const N: usize> NonDeDuplicated<[T; N]> {
    pub const fn as_array_of_cells(&self) -> &[NonDeDuplicated<T>; N] {
        unsafe { core::mem::transmute(self.cell.as_array_of_cells()) }
    }
}

/// For now, [Sync] requires that `T` is both [Sync] AND [Send], following
/// [std::sync::Mutex](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E).
/// However, from <https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html> it seems that `T:
/// Send` may be unnecessary? Please advise.
///
/// Either way, [NonDeDuplicated] exists specifically for static variables. Those get never moved
/// out. So, unlike [std::sync::Mutex], [NonDeDuplicated] itself doesn't need to implement [Send].
unsafe impl<T: Any + Send + Sync> Sync for NonDeDuplicated<T> {}

/// [NonDeDuplicated] is intended for `static` (immutable) variables only. So [Drop::drop] panics in
/// debug builds.
impl<T: Any> Drop for NonDeDuplicated<T> {
    fn drop(&mut self) {
        // If the client uses Box::leak() or friends, then drop() will NOT happen. That is OK: A
        // leaked reference will have static lifetime.
        #[cfg(any(debug_assertions, miri))]
        panic!("Do not use for local variables or on heap. Use for static variables only.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::ptr;

    const fn expect_sync_ref<T: Sync>() {}
    const _: () = expect_sync_ref::<NonDeDuplicated<u8>>();

    #[test]
    #[cfg(any(debug_assertions, miri))]
    #[should_panic(
        expected = "Do not use for local variables or on heap. Use for static variables only."
    )]
    fn drop_panics_in_debug_and_miri() {
        let _: NonDeDuplicated<()> = ().into();
    }

    #[cfg(not(any(debug_assertions, miri)))]
    #[test]
    fn drop_silent_in_release() {
        let _: NonDeDuplicated<()> = ().into();
    }

    const U8_CONST: u8 = b'A';
    static U8_STATIC_1: u8 = b'A';
    static U8_STATIC_2: u8 = b'A';

    #[test]
    fn addresses_unique_between_statics() {
        assert!(!ptr::eq(&U8_STATIC_1, &U8_STATIC_2));
    }

    fn _deref() -> &'static u8 {
        static N: NonDeDuplicated<u8> = NonDeDuplicated::<u8>::new(0);
        &N
    }

    #[test]
    fn deref_of_copy_type() {
        static N: NonDeDuplicated<u8> = NonDeDuplicated::<u8>::new(0);

        let deref = &*N;
        let get = N.get();
        assert!(ptr::eq(deref, get));
    }

    #[cfg(not(any(debug_assertions, miri)))]
    /// In release, [U8_CONST] gets optimized away and points to the same address as
    /// [U8_STATIC_1]!
    #[should_panic(expected = "assertion failed: !ptr::eq(&U8_STATIC_1, &U8_CONST)")]
    #[test]
    fn u8_global_const_and_global_static_release() {
        assert!(!ptr::eq(&U8_STATIC_1, &U8_CONST));
    }
    #[cfg(any(debug_assertions, miri))]
    /// In debug/MIRI, [ARR_CONST] and [ARR_STATIC_1] have unique/separate addresses!
    #[test]
    fn u8_global_const_global_and_static_debug_and_miri() {
        assert!(!ptr::eq(&U8_STATIC_1, &U8_CONST));
    }

    static U8_NDD: NonDeDuplicated<u8> = NonDeDuplicated::new(U8_CONST);
    static U8_NDD_REF: &u8 = U8_NDD.get();
    #[test]
    fn u8_global_const_and_ndd() {
        assert!(!ptr::eq(U8_NDD_REF, &U8_CONST));
        assert!(!ptr::eq(U8_NDD_REF, &U8_STATIC_1));
        assert!(!ptr::eq(U8_NDD_REF, &U8_STATIC_2));
    }

    const STR_CONST_FROM_BYTE_ARRAY: &str = {
        if let Ok(s) = str::from_utf8(&[b'H', b'i']) {
            s
        } else {
            panic!()
        }
    };
    const STR_CONST_FROM_BYTE_STRING: &str = {
        if let Ok(s) = str::from_utf8(b"Hello") {
            s
        } else {
            panic!()
        }
    };

    #[cfg(not(miri))]
    #[test]
    #[should_panic(expected = "assertion failed: !ptr::eq(STR_CONST_FROM_BYTE_ARRAY, \"Hi\")")]
    fn str_global_byte_slice_const_and_local_str_release_and_debug() {
        assert!(!ptr::eq(STR_CONST_FROM_BYTE_ARRAY, "Hi"));
    }
    #[cfg(miri)]
    #[test]
    fn str_global_byte_slice_const_and_local_str_miri() {
        assert!(!ptr::eq(STR_CONST_FROM_BYTE_ARRAY, "Hi"));
    }

    /// This is the same for all three: release, debug AND miri!
    #[test]
    fn str_global_byte_by_byte_const_and_local_static_miri() {
        assert!(ptr::eq(STR_CONST_FROM_BYTE_STRING, "Hello"));
    }

    static STR_STATIC: &str = "Ciao";

    #[cfg(not(miri))]
    #[should_panic(expected = "assertion failed: !ptr::eq(local_const_based_slice, STR_STATIC)")]
    #[test]
    fn str_local_const_based_and_global_static_release_and_debug() {
        str_local_const_based_and_global_static_impl();
    }
    #[cfg(miri)]
    #[test]
    fn str_local_const_based_and_global_static_miri() {
        str_local_const_based_and_global_static_impl();
    }
    fn str_local_const_based_and_global_static_impl() {
        const LOCAL_CONST_ARR: [u8; 4] = [b'C', b'i', b'a', b'o'];
        let local_const_based_slice: &str = str::from_utf8(&LOCAL_CONST_ARR).unwrap();
        assert!(!ptr::eq(local_const_based_slice, STR_STATIC));
    }

    mod cross_module_static {
        pub static STATIC_OPT_U8_A: Option<u8> = Some(b'A');
    }
    mod cross_module_const {
        use core::ptr;
        pub const CONST_OPT_U8_A: Option<u8> = Some(b'A');

        #[cfg(not(any(debug_assertions, miri)))]
        #[test]
        #[should_panic(
            expected = "assertion failed: !ptr::eq(&CONST_OPT_U8_A, &super::cross_module_static::STATIC_OPT_U8_A)"
        )]
        fn option_u8_global_const_global_static_release() {
            assert!(!ptr::eq(
                &CONST_OPT_U8_A,
                &super::cross_module_static::STATIC_OPT_U8_A
            ));
        }
        #[cfg(any(debug_assertions, miri))]
        #[test]
        fn option_u8_global_const_global_static_debug_and_miri() {
            assert!(!ptr::eq(
                &CONST_OPT_U8_A,
                &super::cross_module_static::STATIC_OPT_U8_A
            ));
        }
    }
}
