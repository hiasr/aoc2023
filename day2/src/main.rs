use std::fs;
use std::cmp::max;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut result = 0;
    for line in input.lines() {
        if !line.starts_with("Game") {
            continue;
        }

        let (_id, blue, green, red) = parse_game(line);
        result += blue * green * red;
    }
    println!("{}", result);
}

fn parse_game(game: &str) -> (u64, u64, u64, u64) {
    let (part1, part2) = game.split_once(':').unwrap();
    let id = part1[5..].parse().unwrap();

    let (mut blue, mut green, mut red) = (0, 0, 0);
    for round in part2.split(';') {
        let (blue_round, green_round, red_round) = parse_round(round);
        blue = max(blue, blue_round);
        green = max(green, green_round);
        red = max(red, red_round);
    }
    (id, blue, green, red)
}


fn parse_round(round: &str) -> (u64, u64, u64) {
    let (mut blue, mut green, mut red) = (0, 0, 0);
    for color in round.split(',') {
        let (value, color) = color.trim().split_once(' ').unwrap();
        match color {
            "blue" => blue = value.parse().unwrap(),
            "green" => green = value.parse().unwrap(),
            "red" => red = value.parse().unwrap(),
            _ => panic!("Unknown color: {}", color),
        }
    }
    (blue, green, red)
}
