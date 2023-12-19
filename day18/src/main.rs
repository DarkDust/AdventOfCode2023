use regex::Regex;
use std::time::Instant;

#[derive(Debug)]
enum Error {
    InvalidRegexPattern,
    InvalidInstruction,
}

fn calc_area(nodes: &Vec<(i64, i64)>) -> i64 {
    let mut area = 0;
    let mut perimeter = 0;
    let count = nodes.len();
    for i in 0..count {
        let j = (i + 1) % count;
        let n1 = nodes[i];
        let n2 = nodes[j];
        area += n1.0 * n2.1;
        area -= n1.1 * n2.0;
        perimeter += (((n1.0 - n2.0).pow(2) + (n1.1 - n2.1).pow(2)) as f64).sqrt() as i64;
    }

    area /= 2; // Until here, it's the Shoelace formula.

    // Apply Pick's theorem to get the actual area.
    area += (perimeter / 2) + 1;
    area
}

fn part1(input: &str) -> Result<(), Error> {
    let re = Regex::new(r"^([LRUD]) (\d+) \(#([0-9A-Fa-f]{6})\)")
        .map_err(|_| Error::InvalidRegexPattern)?;

    let mut start = (0, 0);
    let mut nodes = Vec::new();
    for line in input.lines() {
        let matches = re.captures(line).ok_or(Error::InvalidInstruction)?;
        let distance: i64 = matches[2].parse().map_err(|_| Error::InvalidInstruction)?;
        let end = match &matches[1] {
            "L" => (start.0 - distance, start.1),
            "R" => (start.0 + distance, start.1),
            "U" => (start.0, start.1 - distance),
            "D" => (start.0, start.1 + distance),
            _ => panic!("Invalid direction"),
        };

        nodes.push(end);
        start = end;
    }

    println!("Part 1: {}", calc_area(&nodes));
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let re = Regex::new(r"^[LRUD] \d+ \(#([0-9A-Fa-f]{5})([0-3])\)")
        .map_err(|_| Error::InvalidRegexPattern)?;

    let mut start = (0, 0);
    let mut nodes = Vec::new();
    for line in input.lines() {
        let matches = re.captures(line).ok_or(Error::InvalidInstruction)?;
        let distance: i64 =
            i64::from_str_radix(&matches[1], 16).map_err(|_| Error::InvalidInstruction)?;
        let end = match &matches[2] {
            "2" => (start.0 - distance, start.1),
            "0" => (start.0 + distance, start.1),
            "3" => (start.0, start.1 - distance),
            "1" => (start.0, start.1 + distance),
            _ => panic!("Invalid direction"),
        };

        nodes.push(end);
        start = end;
    }

    println!("Part 2: {}", calc_area(&nodes));
    return Ok(());
}

fn main() -> Result<(), Error> {
    let input = include_str!("../rsc/input.txt");

    let start1 = Instant::now();
    part1(input)?;
    println!("Elapsed: {:.2?}\n", start1.elapsed());

    let start2 = Instant::now();
    part2(input)?;
    println!("Elapsed: {:.2?}", start2.elapsed());

    Ok(())
}
