use std::collections::HashSet;
use std::error::Error;
use std::time::Instant;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut total = 0;

    for line in input.lines() {
        let mut temp = line.split(":").skip(1).next().unwrap().split("|");
        let winning = temp
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok());
        let candidates: Vec<i32> = temp
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let matches = winning.filter(|w| candidates.contains(w));
        let count = matches.count();

        if count > 0 {
            total += 1 << (count - 1);
        }
    }

    println!("Total: {}", total);

    return Ok(());
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut cards: Vec<usize> = Vec::new();

    for line in input.lines() {
        let mut temp = line.split(":").skip(1).next().unwrap().split("|");
        let winning: HashSet<i32> = temp
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let candidates: HashSet<i32> = temp
            .next()
            .unwrap()
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        let matches = winning.intersection(&candidates);
        let count = matches.count();

        cards.push(count);
    }

    let mut total = cards.len();
    let mut working: Vec<usize> = (0..total).collect();

    while !working.is_empty() {
        let mut winning: Vec<usize> = Vec::new();

        for index in working {
            let count = cards[index];
            if count > 0 {
                for ni in index + 1..=index + count {
                    winning.push(ni)
                }
            }
        }

        total += winning.len();
        working = winning;
    }

    println!("Total: {}", total);

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
