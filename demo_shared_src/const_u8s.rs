const CROSS_BYTES: &[u8] = b"Cross\0".as_slice();

fn main() {
    callee::print_static_bytes();

    println!("{:?}", CROSS_BYTES.as_ptr());
}
