fn main() {
    callee::print_static_str();

    println!("{:?}", "CROSS".as_bytes().as_ptr());
}
