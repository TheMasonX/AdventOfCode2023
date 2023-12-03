use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    println!("Opened File at path {}\n{}", input_file, input_text);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "";
        let expected = 0;
    }
}
