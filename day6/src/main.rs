use std::fs;
use itertools::Itertools;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut result = 1;

    let (time, distance) = input.lines().map(|x| {
        x.split_once(':')
            .unwrap()
            .1
            .replace(' ', "")
            .parse::<i64>()
            .unwrap()
    }).collect_tuple().unwrap();

    let d =(i64::pow(time,2) - 4*distance) as f64;
    let d = d.sqrt();
    let x1f = ((time as f64) - d)/2.0;
    let x2f = ((time as f64) + d)/2.0;
    let (x1, x2) = (x1f.ceil() as i64, x2f.floor() as i64);

    let mut subresult = x2 - x1 + 1;
    if x1f == x1 as f64 {
        subresult -= 1;
    }
    if x2f == x2 as f64 {
        subresult -= 1;
    }
    result *= subresult;
    println!("{}", result);
}
