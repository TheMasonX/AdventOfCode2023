use day_07::part_one::CardsManager;
use day_07::part_two::CardsManager as CardsManager2;
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    let cards = CardsManager::new(&input_text);
    println!("First Solution: {}", cards.get_output());

    let cards = CardsManager2::new(&input_text);
    println!("Second Solution: {}", cards.get_output());
}
