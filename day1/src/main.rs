use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut result = 0;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let (first, last) = find_first_last_digit(line).unwrap();
        result += first *10 + last;
    }
    println!("Result: {}", result);
}

fn find_first_last_digit(line: &str) -> Option<(u32, u32)>{
    let words = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut digits = Vec::new();
    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_ascii_digit() {
            digits.push(c.to_digit(10).unwrap());
            continue;
        }
        for (j, word) in words.iter().enumerate() {
            if line[i..].starts_with(word) {
                digits.push(j as u32 + 1);
            }
        }
    }
    if !digits.is_empty() {
        return Some((digits[0], digits[digits.len() - 1]));
    }
    None
}
