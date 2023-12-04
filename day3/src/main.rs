use std::collections::{HashMap, HashSet};
use std::fs;


fn main() {
    let lines = fs::read_to_string("input.txt").unwrap().lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    
    let mut gear_map: HashMap<(usize,usize), Vec<usize>> = HashMap::new();

    let mut i = 0;
    let mut j = 0;
    
    while i < lines.len() {
        while j < lines[i].len() {
            if lines[i][j].is_ascii_digit() {
                let mut number = 0;
                let mut gears: HashSet<(usize,usize)> = HashSet::new();
                while j < lines[i].len() && lines[i][j].is_ascii_digit() {
                    number = number * 10 + lines[i][j].to_digit(10).unwrap();
                    gears.extend(&check_gears(&lines, i, j));
                    j += 1;
                }
                for gear in gears {
                    let mut gear_ratios = gear_map.get(&gear).unwrap_or(&Vec::new()).clone();
                    gear_ratios.push(number as usize);
                    gear_map.insert(gear, gear_ratios);
                }
            }
            j += 1;
        }
        j = 0;
        i += 1;
    }
    println!("{:?}", gear_map);
    let result = sum_gear_ratios(&gear_map);
    println!("Result: {}", result);
}

fn sum_gear_ratios(gear_map: &HashMap<(usize,usize), Vec<usize>>) -> usize {
    let mut result = 0;
    for values in gear_map.values() {
        if values.len() != 2 {
            continue;
        }
        result += values[0] * values[1];
    }
    result
}

fn check_gears(lines: &Vec<Vec<char>>, i: usize, j: usize) -> HashSet<(usize,usize)> {
    let mut result: HashSet<(usize,usize)> = HashSet::new();
    for idiff in -1..2 {
        for jdiff in -1..2 {
            if idiff == 0 && jdiff == 0 {
                continue;
            }
            if (i as i32) + idiff < 0 || (j as i32) +jdiff < 0 {
                continue;
            }
            let new_i = ((i as i32) + idiff) as usize;
            let new_j = ((j as i32) + jdiff) as usize;

            if new_i >= lines.len() || new_j >= lines[new_i].len() {
                continue;
            }

            if lines[new_i][new_j] == '*' {
                result.insert((new_i, new_j));
            }
        }
    }
    result
}

