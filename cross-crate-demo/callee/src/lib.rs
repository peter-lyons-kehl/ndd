#[cfg(any(debug_assertions, miri))]
compile_error!(
    "Build only with release profile, so that 'fat lto' has effect. See static*.sh and literal*.sh."
);

// Intentionally NOT public, to see if these `static` variables do get shared cross-crate anyway.
static STATIC_OPT_U8_X: Option<u8> = Some(b'X');
static CROSS: &str = "CROSS";

pub fn print_static_option_u8() {
    println!("{:?}", &STATIC_OPT_U8_X as *const Option<u8>);
}

pub fn print_static_str() {
    println!("{:?}", CROSS.as_bytes().as_ptr());
}

