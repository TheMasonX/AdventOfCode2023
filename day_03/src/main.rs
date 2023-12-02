fn main() {
    let input_file = format!(
        "{}/input.txt",
        std::env::current_dir().unwrap().to_str().unwrap()
    );
    let input_text = match fs::read_to_string(&input_file) {
        Ok(ok) => ok,
        Err(e) => {
            println!("Couldn't open file {}: {:?}", input_file, e);
            return;
        }
    };
}
