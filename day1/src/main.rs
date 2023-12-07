use std::error::Error;
use std::time::Instant;

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut total: u32 = 0;

    for line in input.lines() {
        let parts = line.split("");
        let numbers: Vec<u32> = parts.filter_map(|s| s.parse::<u32>().ok()).collect();
        let res = (numbers.first().unwrap() * 10) + numbers.last().unwrap();
        total = total + res;
    }

    println!("Part 1: {}", total);
    Ok(())
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut total: u32 = 0;

    for line in input.lines() {
        let mut rest = line.to_string();
        let mut numbers: Vec<u32> = Vec::new();

        while !rest.is_empty() {
            if rest.starts_with("0") {
                numbers.push(0);
            } else if rest.starts_with("1") {
                numbers.push(1);
            } else if rest.starts_with("2") {
                numbers.push(2);
            } else if rest.starts_with("3") {
                numbers.push(3);
            } else if rest.starts_with("4") {
                numbers.push(4);
            } else if rest.starts_with("5") {
                numbers.push(5);
            } else if rest.starts_with("6") {
                numbers.push(6);
            } else if rest.starts_with("7") {
                numbers.push(7);
            } else if rest.starts_with("8") {
                numbers.push(8);
            } else if rest.starts_with("9") {
                numbers.push(9);
            } else if rest.starts_with("one") {
                numbers.push(1);
            } else if rest.starts_with("two") {
                numbers.push(2);
            } else if rest.starts_with("three") {
                numbers.push(3);
            } else if rest.starts_with("four") {
                numbers.push(4);
            } else if rest.starts_with("five") {
                numbers.push(5);
            } else if rest.starts_with("six") {
                numbers.push(6);
            } else if rest.starts_with("seven") {
                numbers.push(7);
            } else if rest.starts_with("eight") {
                numbers.push(8);
            } else if rest.starts_with("nine") {
                numbers.push(9);
            }

            rest.remove(0);
        }

        let res = (numbers.first().unwrap() * 10) + numbers.last().unwrap();
        total = total + res;
    }

    println!("Part 2: {}", total);
    Ok(())
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
