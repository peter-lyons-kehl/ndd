const CONST_OPT_U8_X: Option<u8> = Some(b'X');

fn main() {
    cross_crate::print_static_option_u8();

    println!("{:?}", &CONST_OPT_U8_X as *const Option<u8>);
}
