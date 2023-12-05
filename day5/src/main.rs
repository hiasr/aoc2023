use itertools::Itertools;
use std::fs;

#[derive(Debug, Clone, Copy)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn overlaps(&self, other: &Range) -> bool {
        self.start < other.end && self.end > other.start
    }

    fn get_overlap(&self, other: &Range) -> Range {
        Range {
            start: self.start.max(other.start),
            end: self.end.min(other.end),
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    let mut ids = get_seed_ranges(&mut lines);
    println!("{:?}", ids.len());
    let mut new_ids: Vec<Range> = Vec::new();

    for line in lines {
        if line.is_empty() {
            continue;
        }
        if line.contains(':') {
            new_ids.append(&mut ids);
            ids = new_ids;
            new_ids = Vec::new();
            continue;
        }

        let (dest_start, source_start, range) = line
            .split(' ')
            .map(|x| x.parse::<usize>().unwrap())
            .collect_tuple()
            .unwrap();

        let source_range = Range {
            start: source_start,
            end: source_start + range,
        };

        let (has_overlap, mut no_overlap): (Vec<Range>, Vec<Range>) = ids
            .iter()
            .partition(|x| x.overlaps(&source_range));

        for range in has_overlap {
            let overlap = range.get_overlap(&source_range);

            new_ids.push(Range {
                start: overlap.start + dest_start - source_start,
                end: overlap.end + dest_start - source_start,
            });

            let (left, right) = if range.start < overlap.start {
                (
                    Range {
                        start: range.start,
                        end: overlap.start,
                    },
                    Some(Range {
                        start: overlap.end,
                        end: range.end,
                    }),
                )
            } else {
                (
                    Range {
                        start: overlap.end,
                        end: range.end,
                    },
                    None,
                )
            };

            if let Some(right) = right {
                no_overlap.push(right);
            }
            if left.start < left.end {
                no_overlap.push(left);
            }
        }

        ids = no_overlap;
    }
    ids.append(&mut new_ids);
    println!("{:?}", ids.iter().map(|x| x.start).min().unwrap());
}

fn get_seed_ranges(lines: &mut std::str::Lines) -> Vec<Range> {
    lines
        .next()
        .unwrap()
        .split_once(':')
        .unwrap()
        .1
        .trim()
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .tuples()
        .map(|(start, length)| Range { start, end: start + length })
        .collect::<Vec<Range>>()
}

