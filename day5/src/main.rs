use std::collections::HashMap;
use std::error::Error;
use std::time::Instant;

struct SeedRange {
    source: i64,
    dest: i64,
    length: i64,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct Index {
    from: String,
    to: String,
}

impl SeedRange {
    fn map(&self, value: i64) -> Option<i64> {
        if value < self.source || value >= self.source + self.length {
            return None;
        }

        Some(self.dest + (value - self.source))
    }
}

fn map_value(value: i64, ranges: &Vec<SeedRange>) -> i64 {
    for range in ranges {
        if let Some(mapped) = range.map(value) {
            return mapped;
        }
    }

    value
}

fn find_mapping<'a>(
    source: &str,
    mappings: &'a HashMap<Index, Vec<SeedRange>>,
) -> Option<(&'a Index, &'a Vec<SeedRange>)> {
    for (index, ranges) in mappings {
        if index.from == source {
            return Some((index, ranges));
        }
    }

    None
}

fn find_location(seed: i64, mappings: &HashMap<Index, Vec<SeedRange>>) -> i64 {
    let mut source = "seed";
    let mut value = seed;

    while let Some((index, ranges)) = find_mapping(source, mappings) {
        value = map_value(value, ranges);
        if index.to == "location" {
            return value;
        }

        source = &index.to;
    }

    panic!("No end!")
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut mappings: HashMap<Index, Vec<SeedRange>> = HashMap::new();
    let mut seeds: Vec<i64> = Vec::new();
    let mut index: Index = Index {
        from: "".to_string(),
        to: "".to_string(),
    };

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds:") {
            seeds = line[7..]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            continue;
        }

        if line.ends_with("map:") {
            let mut parts = line.split(' ').next().unwrap().split('-');
            let from = parts.next().unwrap();
            _ = parts.next();
            let to = parts.next().unwrap();
            index = Index {
                from: from.to_string(),
                to: to.to_string(),
            };
            continue;
        }

        let mut range_parts = line.split_whitespace().map(|s| s.parse().unwrap());
        let dest = range_parts.next().unwrap();
        let source = range_parts.next().unwrap();
        let length = range_parts.next().unwrap();
        mappings
            .entry(index.clone())
            .or_insert(Vec::new())
            .push(SeedRange {
                source: source,
                dest: dest,
                length: length,
            });
    }

    let mut lowest = i64::max_value();
    for seed in seeds {
        let loc = find_location(seed, &mappings);
        lowest = lowest.min(loc);
    }

    println!("Part 1: {lowest}");
    return Ok(());
}

// Meh. Takes 3min to complete on my machine. Too lazy to optimize, it spit out the solution in
// an acceptable time.
// Optimization idea: process the mappings to we end up with a single soil to location mapping.
// For that, ranges need to be split and shifted.
fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut mappings: HashMap<Index, Vec<SeedRange>> = HashMap::new();
    let mut seeds: Vec<(i64, i64)> = Vec::new();
    let mut index: Index = Index {
        from: "".to_string(),
        to: "".to_string(),
    };

    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        if line.starts_with("seeds:") {
            let intermediate: Vec<i64> = line[7..]
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            seeds = intermediate.chunks(2).map(|c| (c[0], c[1])).collect();
            continue;
        }

        if line.ends_with("map:") {
            let mut parts = line.split(' ').next().unwrap().split('-');
            let from = parts.next().unwrap();
            _ = parts.next();
            let to = parts.next().unwrap();
            index = Index {
                from: from.to_string(),
                to: to.to_string(),
            };
            continue;
        }

        let mut range_parts = line.split_whitespace().map(|s| s.parse().unwrap());
        let dest = range_parts.next().unwrap();
        let source = range_parts.next().unwrap();
        let length = range_parts.next().unwrap();
        mappings
            .entry(index.clone())
            .or_insert(Vec::new())
            .push(SeedRange {
                source: source,
                dest: dest,
                length: length,
            });
    }

    let mut lowest = i64::max_value();
    for seed_range in seeds {
        for seed in seed_range.0..(seed_range.0 + seed_range.1) {
            let loc = find_location(seed, &mappings);
            lowest = lowest.min(loc);
        }
    }

    println!("Part 2: {lowest}");
    return Ok(());
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../rsc/input.txt");

    let start1 = Instant::now();
    part1(input)?;
    println!("Elapsed: {:.2?}\n", start1.elapsed());

    let start2 = Instant::now();
    part2(input)?;
    println!("Elapsed: {:.2?}", start2.elapsed());

    Ok(())
}
