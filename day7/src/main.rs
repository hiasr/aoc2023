use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut hands: Vec<(&str, u32)> = input.lines()
        .map(|line| {
            let (hand, points) = line.split_once(' ').unwrap();
            (hand, points.parse::<u32>().unwrap())
        }).collect();
    hands.sort_by(compare_hands);
    let result = hands.iter().enumerate().fold(0, |acc, (i, (_, points))| {
        acc + (i as u32 + 1) * points
    });
    println!("{}", result);
}

fn compare_hands((hand1,_points1): &(&str, u32), (hand2,_points2): &(&str, u32)) -> Ordering {
    println!("{} {}", hand1, hand2);
    let mut h1map = HashMap::new();
    hand1.chars().for_each(|c| {
        let count = h1map.entry(c).or_insert(0);
        *count += 1;
    });

    let mut h2map = HashMap::new();
    hand2.chars().for_each(|c| {
        let count = h2map.entry(c).or_insert(0);
        *count += 1;
    });

    let h1jokers = *h1map.get(&'J').unwrap_or(&0);
    let h2jokers = *h2map.get(&'J').unwrap_or(&0);

    h1map.insert('J', 0);
    h2map.insert('J', 0);

    let mut counts1: Vec<u32> = h1map.values().copied().collect::<Vec<u32>>();
    counts1.sort();
    counts1.reverse();
    let mut counts2: Vec<u32> = h2map.values().copied().collect::<Vec<u32>>();
    counts2.sort();
    counts2.reverse();

    counts1[0] += h1jokers;
    counts2[0] += h2jokers;

    match counts1[0].cmp(&counts2[0]) {
        Ordering::Greater => return Ordering::Greater,
        Ordering::Less => return Ordering::Less,
        Ordering::Equal => {},
    }

    match counts1[0] {
        5 => highest_first_card(hand1, hand2),
        4 => highest_first_card(hand1, hand2),
        3 | 2 => {
            match counts1[1].cmp(&counts2[1]) {
                Ordering::Greater => Ordering::Greater,
                Ordering::Less => Ordering::Less,
                Ordering::Equal => highest_first_card(hand1, hand2), 
            }
        },
        1 => highest_first_card(hand1, hand2),
        _ => panic!("Invalid hand"),
    }
}

fn highest_first_card(hand1: &str, hand2: &str) -> Ordering {
    for (c1, c2) in hand1.chars().zip(hand2.chars()) {
        match compare_card(c1, c2) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {},
        }
    }
    Ordering::Equal
}

fn compare_card(card1: char, card2: char) -> Ordering {
    let order = ['A','K','Q','T','9','8','7','6','5','4','3','2','J'];
    let index1 = order.iter().position(|&c| c == card1).unwrap();
    let index2 = order.iter().position(|&c| c == card2).unwrap();

    index2.cmp(&index1)
}
