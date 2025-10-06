// Intentionally NOT public, to see if it gets shared cross-crate anyway.
static STATIC_OPT_U8_X: Option<u8> = Some(b'X');
static CROSS: &'static str = "CROSS";

pub fn print_static_option_u8() {
    println!("{:?}", &STATIC_OPT_U8_X as *const Option<u8>);
}

pub fn print_static_str() {
    println!("{:?}", CROSS.as_bytes().as_ptr());
}
