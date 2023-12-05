use day_04::*;
use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();
    let mut card_manager = CardManager::new(&input_text);
    println!("Second Total: {}", card_manager.waterfall_rounds());

    // let total = card_manager.cards.values().map(|x| x.score).sum::<i32>();
    // println!("First Total: {}", total);
    println!("Second Total: {}", card_manager.play_all_rounds());
}
