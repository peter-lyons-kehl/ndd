// OVerride links, so that rustdoc can point them locally (or to docs.io, if run on docs.io).
//! [`src/lib.rs` -> `addresses_unique_between_statics()`]: https://github.com/peter-lyons-kehl/ndd/blob/main/src/lib.rs#L246
#![doc = include_str!("../README.md")]
#![cfg_attr(not(any(doc, test)), no_std)]
#![allow(incomplete_features)]

use core::any::Any;
use core::cell::Cell;
use core::ffi::CStr;
use core::marker::PhantomData;

/// A zero-cost  wrapper guaranteed not to share its memory location with any other valid (in-scope)
/// variable (even `const` equal to the inner value). Use for `static` variables that have their
/// addresses compared with [core::ptr::eq].
///
/// It has same size, layout and alignment as type parameter `OWN`.
///
/// `T` must implement [Any]. That is automatic for any types that don't have any lifetimes (other
/// than `'static`). This requirement gives an earlier error when `NonDeDuplicated` is used other
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
/// But, by requiring [NonDeDuplicatedFlexible]'s generic parameter `OWN` (or [NonDeDuplicated]'s
/// generic parameter `T`) to implement [Any] the first example above fails, too. That prevents
/// mistakes earlier.
///
/// Do not use [NonDeDuplicatedFlexible] directly. Instead, use [NonDeDuplicated],
/// [NonDeDuplicatedStr] and [NonDeDuplicatedCStr].
#[repr(transparent)]
pub struct NonDeDuplicatedFlexible<OWN: Any + Send + Sync, TO: Any + Send + Sync + ?Sized> {
    cell: Cell<OWN>,
    _t: PhantomData<TO>,
}

/// For non-de-duplicated objects stored in `static` variables. NOT for string slices - for those
/// use [NonDeDuplicatedStr] and [NonDeDuplicatedCStr].
#[allow(type_alias_bounds)]
pub type NonDeDuplicated<T: Any + Send + Sync> = NonDeDuplicatedFlexible<T, T>;

impl<T: Any + Send + Sync> NonDeDuplicated<T> {
    /// Construct a new instance.
    pub const fn new(value: T) -> Self {
        Self {
            //Using core::hint::black_box() seems unnecessary.
            //cell: Cell::new(core::hint::black_box(value)),
            cell: Cell::new(value),
            _t: PhantomData,
        }
    }

    /// Get a reference.
    pub const fn get(&self) -> &T {
        let ptr = self.cell.as_ptr();
        unsafe { &*ptr }
    }
}

/// Separate from [bytes_to_array], so that we help monomorphization surface area to be smaller.
const fn copy_bytes_to_array(to: &mut [u8], from: &[u8], len: usize) {
    if from.len() > len {
        let msg = match from.len() - len {
            1 => "Target length is 1 byte too small.",
            2 => "Target length is 2 bytes too small.",
            3 => "Target length is 3 bytes too small.",
            4 => "Target length is 4 bytes too small.",
            _ => "Target length is more than 4 bytes too small.",
        };
        panic!("{}", msg)
    }
    if from.len() < len {
        let msg = match len - from.len() {
            1 => "Target length is 1 byte too large.",
            2 => "Target length is 2 bytes too large.",
            3 => "Target length is 3 bytes too large.",
            4 => "Target length is 4 bytes too large.",
            _ => "Target length is more than 4 bytes too large.",
        };
        panic!("{}", msg)
    }
    if to.len() != len {
        panic!("Target slice length differs to the specified length.")
    }

    let mut i = 0;
    while i < len {
        to[i] = from[i];
        i += 1;
    }
}

const fn bytes_to_array<const N: usize>(bytes: &[u8]) -> [u8; N] {
    let mut arr = [0u8; N];
    copy_bytes_to_array(&mut arr, bytes, N);
    arr
}

/// For non-de-duplicated string slices stored in `static` variables.
pub type NonDeDuplicatedStr<const N: usize> = NonDeDuplicatedFlexible<[u8; N], str>;
impl<const N: usize> NonDeDuplicatedStr<N> {
    /// Construct a new instance.
    pub const fn new(s: &str) -> Self {
        Self {
            cell: Cell::new(bytes_to_array(s.as_bytes())),
            _t: PhantomData,
        }
    }

    /// Get a reference.
    ///
    /// Implementation details: Since this type, and this function, is intended to be used for
    /// `static` variables only, speed doesn't matter here. So, we use [core::str::from_utf8]
    /// (instead of [core::str::from_utf8_unchecked]).
    pub const fn get(&self) -> &str {
        let ptr = self.cell.as_ptr();
        let bytes = unsafe { &*ptr };
        match core::str::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    }
}

/// For non-de-duplicated string slices stored in `static` variables.
pub type NonDeDuplicatedCStr<const N: usize> = NonDeDuplicatedFlexible<[u8; N], CStr>;
impl<const N: usize> NonDeDuplicatedCStr<N> {
    /// Construct a new instance.
    pub const fn new(s: &CStr) -> Self {
        Self {
            cell: Cell::new(bytes_to_array(s.to_bytes())),
            _t: PhantomData,
        }
    }

    /// The given `arr` must be a well-formed C string, that is,
    /// - **not** containing any internal NUL bytes, and
    /// - end with a NUL byte, like `b"abc\0"`.
    pub const fn new_from_bytes(arr: [u8; N]) -> Self {
        // Validate early, rather than waiting for validation by .get()
        let _ = core::hint::black_box(CStr::from_bytes_with_nul(&arr));
        Self {
            cell: Cell::new(arr),
            _t: PhantomData,
        }
    }

    ///  The `given &`[str] must, like C string, not contain any internal NUL bytes. However, do
    ///  **not** include the trailing NUL byte - that is added automatically.
    pub const fn new_from_str(s: &str) -> Self {
        let mut arr = [0u8; N];
        if let Some((_, sub_slice)) = (&mut arr).split_last_mut() {
            crate::copy_bytes_to_array(sub_slice, s.as_bytes(), s.len());
        } else {
            unreachable!()
        }
        Self::new_from_bytes(arr)
    }

    /// Get a reference.
    ///
    /// Implementation details: Since this type, and this function, is intended to be used for
    /// `static` variables only, speed doesn't matter here. So, we use [CStr::from_bytes_with_nul]
    /// (instead of [CStr::from_bytes_with_nul_unchecked]).
    pub const fn get(&self) -> &CStr {
        let ptr = self.cell.as_ptr();
        let bytes = unsafe { &*ptr };
        match CStr::from_bytes_with_nul(bytes) {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    }
}

/// For now, [Sync] (and [NonDeDuplicatedFlexible] in general) requires that `OWN` is both [Sync]
/// AND [Send], following
/// [std::sync::Mutex](https://doc.rust-lang.org/nightly/std/sync/struct.Mutex.html#impl-Sync-for-Mutex%3CT%3E).
/// However, from <https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html> it seems that `T:
/// Send` may be unnecessary? Please advise.
///
/// Either way, [NonDeDuplicated], [NonDeDuplicatedStr] and [NonDeDuplicatedCStr] (and underlying
/// [NonDeDuplicatedFlexible]) exist specifically for static variables. Those get never moved out.
/// So, (unlike [std::sync::Mutex]) they do **not** need to implement [Send].
///
/// Also, unclear if `TO` needs to be [Send] and [Sync].
unsafe impl<OWN: Any + Send + Sync, TO: Any + Send + Sync + ?Sized> Sync
    for NonDeDuplicatedFlexible<OWN, TO>
{
}

/// [NonDeDuplicated] and friends are intended for `static` (immutable) variables only. So
/// [Drop::drop] panics in debug/miri builds.
impl<OWN: Any + Send + Sync, TO: Any + Send + Sync + ?Sized> Drop
    for NonDeDuplicatedFlexible<OWN, TO>
{
    fn drop(&mut self) {
        // If the client uses Box::leak() or friends, then drop() will NOT happen. That is OK: A
        // leaked reference will have static lifetime.
        #[cfg(any(debug_assertions, miri))]
        panic!("Do not use for local variables, const, or on heap. Use for static variables only.")
    }
}

#[cfg(test)]
mod tests_shared {
    pub const STR_CONST_FROM_BYTE_ARRAY_HI: &str = {
        match str::from_utf8(&[b'H', b'i']) {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    };

    pub const STR_CONST_FROM_BYTE_STRING_HELLO: &str = {
        match str::from_utf8(b"Hello") {
            Ok(s) => s,
            Err(_) => unreachable!(),
        }
    };
}

/// These tests don't actually test `ndd`, but they test the behavior **without** `ndd`.
#[cfg(test)]
mod tests_behavior_without_ndd {
    use crate::tests_shared::{STR_CONST_FROM_BYTE_ARRAY_HI, STR_CONST_FROM_BYTE_STRING_HELLO};
    use core::ptr;

    const U8_CONST: u8 = b'A';
    static U8_STATIC_1: u8 = b'A';
    static U8_STATIC_2: u8 = b'A';

    #[test]
    fn addresses_unique_between_statics() {
        assert!(!ptr::eq(&U8_STATIC_1, &U8_STATIC_2));
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

    #[cfg(not(miri))]
    #[test]
    #[should_panic(expected = "assertion failed: !ptr::eq(STR_CONST_FROM_BYTE_ARRAY_HI, \"Hi\")")]
    fn str_global_byte_slice_const_and_local_str_release_and_debug() {
        assert!(!ptr::eq(STR_CONST_FROM_BYTE_ARRAY_HI, "Hi"));
    }
    #[cfg(miri)]
    #[test]
    fn str_global_byte_slice_const_and_local_str_miri() {
        assert!(!ptr::eq(STR_CONST_FROM_BYTE_ARRAY_HI, "Hi"));
    }

    /// This is the same for all three: release, debug AND miri!
    #[test]
    fn str_global_byte_by_byte_const_and_local_static() {
        assert!(ptr::eq(STR_CONST_FROM_BYTE_STRING_HELLO, "Hello"));
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

#[cfg(test)]
mod tests_behavior_with_ndd {
    use super::*;
    use crate::tests_shared::{STR_CONST_FROM_BYTE_ARRAY_HI, STR_CONST_FROM_BYTE_STRING_HELLO};
    use core::ptr;

    const U8_CONST: u8 = b'A';
    static U8_STATIC_1: u8 = b'A';
    static U8_STATIC_2: u8 = b'A';

    const fn expect_sync_ref<T: Sync>() {}
    const _: () = expect_sync_ref::<NonDeDuplicated<u8>>();

    static U8_NDD: NonDeDuplicated<u8> = NonDeDuplicated::new(U8_CONST);
    static U8_NDD_REF: &u8 = U8_NDD.get();
    #[test]
    fn u8_global_const_and_ndd() {
        assert!(!ptr::eq(U8_NDD_REF, &U8_CONST));
        assert!(!ptr::eq(U8_NDD_REF, &U8_STATIC_1));
        assert!(!ptr::eq(U8_NDD_REF, &U8_STATIC_2));
    }

    static STR_NDD_HI: NonDeDuplicatedStr<5> = NonDeDuplicatedStr::new("Hello");
    #[test]
    fn str_ndd_hi() {
        assert!(!ptr::eq(STR_NDD_HI.get(), "Hi"));
        assert!(!ptr::eq(STR_NDD_HI.get(), STR_CONST_FROM_BYTE_ARRAY_HI));
        assert!(!ptr::eq(STR_NDD_HI.get(), STR_CONST_FROM_BYTE_STRING_HELLO));
    }

    static STR_NDD_CIAO: NonDeDuplicatedStr<4> = NonDeDuplicatedStr::new("Ciao");
    #[test]
    fn str_local_const_based_and_str_ndd() {
        const LOCAL_CONST_ARR: [u8; 4] = [b'C', b'i', b'a', b'o'];
        let local_const_based_slice: &str = str::from_utf8(&LOCAL_CONST_ARR).unwrap();
        assert!(!ptr::eq(local_const_based_slice, STR_NDD_CIAO.get()));
    }

    #[test]
    #[cfg(any(debug_assertions, miri))]
    #[should_panic(
        expected = "Do not use for local variables, const, or on heap. Use for static variables only."
    )]
    fn drop_panics_in_debug_and_miri() {
        let _: NonDeDuplicated<()> = NonDeDuplicated::new(());
    }

    #[cfg(not(any(debug_assertions, miri)))]
    #[test]
    fn drop_silent_in_release() {
        let _: NonDeDuplicated<()> = NonDeDuplicated::new(());
    }
}
