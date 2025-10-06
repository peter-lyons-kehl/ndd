const CROSS: &str = "CROSS";

fn main() {
    cross_crate::print_static_str();

    println!("{:?}", CROSS.as_bytes().as_ptr());
}
