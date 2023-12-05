use day_05::Almanac;
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    let mut almanac = Almanac::new(&input_text);
    let first = *almanac.seeds_to_soil().iter().min().unwrap();
    println!("First Solution: {}", first);
    let second = almanac.seed_ranges_to_soil();
    println!("Second Solution: {}", second);
}
