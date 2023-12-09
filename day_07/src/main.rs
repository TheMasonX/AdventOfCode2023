use day_07::CardsManager;
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    let cards = CardsManager::new(&input_text);
    println!("First Solution: {}", cards.get_output());
}
