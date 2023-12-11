use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
enum Error {}

fn distance(p0: &(usize, usize), p1: &(usize, usize)) -> usize {
    (p0.0.max(p1.0) - p0.0.min(p1.0)) + (p0.1.max(p1.1) - p0.1.min(p1.1))
}

fn process(input: &str, gap_size: usize) -> Result<usize, Error> {
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                galaxies.push((x, y));
                max_x = max_x.max(x);
                max_y = max_y.max(y);
            }
        }
    }

    let all_x: HashSet<&usize> = galaxies.iter().map(|(x, _)| x).collect();
    let all_y: HashSet<&usize> = galaxies.iter().map(|(_, y)| y).collect();
    let mut delta_x: Vec<usize> = Vec::new();
    let mut delta_y: Vec<usize> = Vec::new();

    let mut gaps = 0;
    for x in 0..=max_x {
        if !all_x.contains(&x) {
            gaps += gap_size;
        }
        delta_x.push(gaps);
    }

    gaps = 0;
    for y in 0..=max_y {
        if !all_y.contains(&y) {
            gaps += gap_size;
        }
        delta_y.push(gaps);
    }

    let expanded_galaxies: Vec<(usize, usize)> = galaxies
        .iter()
        .map(|(x, y)| (x + delta_x[*x], y + delta_y[*y]))
        .collect();

    let sum: usize = expanded_galaxies
        .iter()
        .combinations(2)
        .map(|c| distance(c[0], c[1]))
        .sum();

    Ok(sum)
}

fn part1(input: &str) -> Result<(), Error> {
    println!("Part 1: {}", process(input, 1)?);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    // Beware of Obiwan… (off-by-one)
    println!("Part 2: {}", process(input, 1_000_000 - 1)?);
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
