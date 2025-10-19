#[cfg(any(miri))]
compile_error!(
    "Build only with release or modified dev profile, so that 'fat lto' has effect. See ../../../demo_shared_scripts/"
);

// Intentionally NOT public, to see if these `static` variables do get shared cross-crate anyway.
static STATIC_OPT_U8_X: Option<u8> = Some(b'X');
static CROSS_STR: &str = "CROSS";
static CROSS_BYTES: &[u8] = b"Cross\0".as_slice();

pub fn print_static_option_u8() {
    println!("{:?}", &STATIC_OPT_U8_X as *const Option<u8>);
}

pub fn print_static_str() {
    println!("{:?}", CROSS_STR.as_bytes().as_ptr());
}

pub fn print_static_bytes() {
    println!("{:?}", CROSS_BYTES.as_ptr());
}
