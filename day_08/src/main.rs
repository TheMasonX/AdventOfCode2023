use crate::StructA;
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    let s = StructA::new(input_text);
    println!("First Solution: {}", s.get_output());
}
