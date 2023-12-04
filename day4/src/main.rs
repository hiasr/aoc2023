use std::collections::{HashSet, HashMap};
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();

    for line in input.lines() {
        let (id, card) = line.split_once(':').unwrap();
        println!("{}", id);
        let id = id.split(' ').filter(|x| !x.is_empty()).nth(1).unwrap().parse::<u32>().unwrap(); 
        let (winning, nums) = card.split_once('|').unwrap();
        let winning = winning
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let matching_nums = nums
            .trim()
            .split(' ')
            .filter(|x| !x.is_empty())
            .map(|x| x.trim().parse::<u32>().unwrap())
            .filter(|x| winning.contains(x))
            .collect();
        map.insert(id, matching_nums);
    }

    let mut cache: HashMap<u32, u32> = HashMap::new();
    let result: u32 = map.keys().map(|x| get_scratch_count(*x, &map, &mut cache)).sum();


    println!("{}", result);
}

fn get_scratch_count(
    id: u32,
    map: &HashMap<u32, Vec<u32>>,
    cache: &mut HashMap<u32, u32>,
) -> u32 {
    if let Some(count) = cache.get(&id) {
        return *count;
    }

    let mut count = 1;
    for num in id+1..id+(map.get(&id).unwrap().len() as u32)+1 {
        count += get_scratch_count(num, map, cache);
    }
    cache.insert(id, count);
    count
}
