use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut result = 0;

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let sequence: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        let mut sequences = vec![sequence];

        while !sequences.last().unwrap().iter().all(|&x| x == 0) {
            let next_seq = sequences
                .last()
                .unwrap()
                .windows(2)
                .map(|x| x[1] - x[0])
                .collect::<Vec<i32>>();
            sequences.push(next_seq.clone());
        }
        result += sequences.iter().rev().fold(0, |acc, x| x.first().unwrap() - acc);
    }
    println!("{}", result);
}
