use day_06::RaceSet;
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    let set1 = RaceSet::new(&input_text);
    let set2 = RaceSet::new_part_2(&input_text);
    println!("First Solution: {}", set1.get_output());
    println!("Second Solution: {}", set2.get_output());
}
