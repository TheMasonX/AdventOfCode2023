use std::fs;

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

    let mut first_total: i32 = 0;
    let mut second_total: i32 = 0;
    println!("{} Lines", input_text.lines().count());
    input_text.lines().for_each(|s| {
        first_total += get_num_chars(s);
        second_total += get_num_words(s);
    });
    println!("Totals are: {} and {}", first_total, second_total);
}

pub fn get_num_chars(s: &str) -> i32 {
    let mut left = -1;
    let mut right = -1;

    for c in s.chars() {
        match c {
            '0'..='9' => {
                right = c.to_digit(10).unwrap() as i32;
                if left == -1 {
                    left = right;
                }
            }
            _ => continue,
        }
    }
    if left == -1 {
        println!("Found no nums in {}", s);
        return 0;
    }

    left * 10 + right
}

pub struct NumberMatch {
    input: String,
    output: i32,
}

impl NumberMatch {
    pub fn new(input_str: &str, output: i32) -> Self {
        Self {
            input: String::from(input_str),
            output,
        }
    }

    pub fn matches(&self, input_str: &str) -> bool {
        input_str.starts_with(&self.input)
    }
}

pub fn get_num_words(s: &str) -> i32 {
    let mut left = -1;
    let mut right = -1;
    let string = s.to_string();

    let mut matches: Vec<NumberMatch> = vec![
        NumberMatch::new("one", 1),
        NumberMatch::new("two", 2),
        NumberMatch::new("three", 3),
        NumberMatch::new("four", 4),
        NumberMatch::new("five", 5),
        NumberMatch::new("six", 6),
        NumberMatch::new("seven", 7),
        NumberMatch::new("eight", 8),
        NumberMatch::new("nine", 9),
    ];

    for i in 1..=9 {
        matches.push(NumberMatch::new(&i.to_string(), i))
    }

    for index in 0..=s.len() {
        let slice = &string[index..s.len()].to_string();
        for m in matches.iter().filter(|m| m.matches(slice)) {
            right = m.output;
            if left == -1 {
                left = right;
            }
        }
    }

    if left == -1 {
        println!("Found no nums in {}", s);
        return 0;
    }
    left * 10 + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_chars() {
        let inputs = ["1abc2", "pqr3stu8vwx", "a1b2c3d4e5f", "treb7uchet"];
        let outputs = [12, 38, 15, 77];
        let total = 142;

        let mut calc_total = 0;
        for (input, o) in inputs.iter().zip(outputs.iter()) {
            let calc_output = get_num_chars(input);
            calc_total += calc_output;
            println!("{} -> {}, expected {}", input, calc_output, o);
            assert_eq!(calc_output, *o);
        }
        assert_eq!(calc_total, total);
    }

    #[test]
    fn test_num_words() {
        let inputs = [
            "two1nine",
            "eightwothree",
            "abcone2threexyz",
            "xtwone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
            "eighthree", //Had to add this because it was an edge case not covered by the supplied tests
        ];
        let outputs = [29, 83, 13, 24, 42, 14, 76, 83];
        let total = 364;

        let mut calc_total = 0;
        for (input, o) in inputs.iter().zip(outputs.iter()) {
            let calc_output = get_num_words(input);
            calc_total += calc_output;
            println!("{} -> {}, expected {}", input, calc_output, o);
            assert_eq!(calc_output, *o);
        }
        assert_eq!(calc_total, total);
    }
}
