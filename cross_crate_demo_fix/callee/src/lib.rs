#[cfg(any(miri))]
compile_error!(
    "Build only with release or modified dev profile, so that 'fat lto' has effect. See ../../../cross_crate_shared_scripts/"
);

use core::ffi::CStr;
use ndd::{NonDeDuplicated, NonDeDuplicatedCStr, NonDeDuplicatedStr};

// Intentionally NOT public, to see if these `static` variables do get shared cross-crate anyway.
static STATIC_OPT_U8_X_NDD: NonDeDuplicated<Option<u8>> = NonDeDuplicated::new(Some(b'X'));
static STATIC_OPT_U8_X: &Option<u8> = STATIC_OPT_U8_X_NDD.get();

static CROSS_STR_NDD: NonDeDuplicatedStr<5> = NonDeDuplicatedStr::new("CROSS");
static CROSS_STR: &str = CROSS_STR_NDD.get();

static CROSS_CSTR_NDD: NonDeDuplicatedCStr<6> = NonDeDuplicatedCStr::new_from_bytes(*b"Cross\0");
static CROSS_CSTR: &CStr = CROSS_CSTR_NDD.get();

pub fn print_static_option_u8() {
    println!("{:?}", STATIC_OPT_U8_X as *const Option<u8>);
}

pub fn print_static_str() {
    println!("{:?}", CROSS_STR.as_bytes().as_ptr());
}

pub fn print_static_bytes() {
    println!("{:?}", CROSS_CSTR.as_ptr());
}
