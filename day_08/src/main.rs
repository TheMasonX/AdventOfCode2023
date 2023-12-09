use day_08::part_one::StructA;
use day_08::part_two::StructA as StructA2;
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    let a = StructA::new(&input_text);
    println!("First Solution: {}", a.get_output());

    let b = StructA2::new(&input_text);
    println!("Second Solution: {}", b.get_output());
}
