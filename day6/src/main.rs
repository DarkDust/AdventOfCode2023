use std::error::Error;
use std::time::Instant;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let times = lines[0]
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .flat_map(|s| s.parse::<i64>());
    let distances = lines[1]
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .flat_map(|s| s.parse::<i64>());
    let mut result: usize = 1;

    for (time, distance) in times.zip(distances) {
        let candidates = (1..time).map(|t| (time - t) * t);
        let valid = candidates.filter(|d| *d > distance).count();
        result = result * valid;
    }

    println!("Part 1: {result}");
    return Ok(());
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let lines: Vec<&str> = input.lines().collect();
    let times: Vec<&str> = lines[0]
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    let time: i64 = times.join("").parse().unwrap();
    let distances: Vec<&str> = lines[1]
        .split(":")
        .skip(1)
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .collect();
    let distance: i64 = distances.join("").parse().unwrap();
    let mut result: usize = 1;

    let candidates = (1..time).map(|t| (time - t) * t);
    let valid = candidates.filter(|d| *d > distance).count();
    result = result * valid;

    println!("Part 2: {result}");
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
