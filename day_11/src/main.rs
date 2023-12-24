use day_11::part_one::StructA;
use day_11::part_two::StructA as StructA2;
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    let cards = StructA::new(&input_text);
    println!("First Solution: {}", cards.get_output());

    let cards = StructA2::new(&input_text);
    println!("Second Solution: {}", cards.get_output());
}
