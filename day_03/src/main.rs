use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();
    println!("Opened File with contents:\n{}", input_text);
}
