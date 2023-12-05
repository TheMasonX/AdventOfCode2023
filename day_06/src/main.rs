use tmx_utils::string_ext;

fn main() {
    let input_text = string_ext::read_local_file("input.txt").unwrap();

    println!("First Solution: {}", 0);
    println!("Second Solution: {}", 0);
}

#[derive(Debug)]
pub struct StructA {}

impl StructA {
    pub fn new(input: &str) -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input_text = "";
        let expected = 0;
        let mut actual = 0;

        let s = StructA::new(input_text);

        assert_eq!(expected, actual);
    }
}
