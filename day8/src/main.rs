use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let steps = lines
        .next()
        .unwrap()
        .chars();
    let step_length = steps.clone().count();
    let mut steps = steps.cycle();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        if line.is_empty() {
            continue;
        }
        let (start, dests) = line.split_once('=').unwrap();
        let (dest1, dest2) = dests
            .split(',')
            .map(|s| {
                s.trim()
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>()
            })
            .collect_tuple()
            .unwrap();

        let start = start.trim();
        map.insert(start.to_owned(), (dest1, dest2));
    }

    let nodes = map
        .keys()
        .to_owned()
        .filter(|s| s.ends_with('A'))
        .map(|s| s.to_owned())
        .collect::<Vec<_>>();
    println!("Nodes: {:?}", nodes);
    let mut cycle_lengths: Vec<usize> = Vec::new();

    for starting_node in nodes.clone().iter() {
        let mut step_count = 0;
        let mut node = starting_node.as_str();
        let mut steps = steps.clone();
        while !node.ends_with('Z') {
            let step = steps.next().unwrap();
            let (left, right) = map.get(node).unwrap();
            match step {
                'L' => {
                    node = left;
                }
                'R' => {
                    node = right;
                }
                _ => panic!("Invalid step"),
            }
            step_count += 1;
        }
        println!("Steps: {}", step_count);
        cycle_lengths.push(step_count);
    }
    println!("Cycle lengths: {:?}", cycle_lengths);
    let lcm = cycle_lengths.iter().fold(1, |acc, x| acc * x / step_length);
    println!("LCM: {}", lcm*step_length);
}
