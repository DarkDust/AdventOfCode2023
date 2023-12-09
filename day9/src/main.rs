use itertools::Itertools;
use std::time::Instant;

#[derive(Debug)]
enum Error {
    NotANumber(String),
    OutOfBounds,
}

// Function/closure that combines the list of numbers of a "level" with the difference passed from
// previous level.
type Processor = fn(&Vec<i32>, i32) -> Result<i32, Error>;

// Prses the input, recurses on each line with the passed processor.
fn parse_and_process(input: &str, processor: Processor) -> Result<i32, Error> {
    let result: Result<Vec<i32>, Error> = input
        .lines()
        .map(|line| {
            // Parse line into a list of numbers.
            let numbers: Result<Vec<i32>, Error> = line
                .split_whitespace()
                .map(|s| s.parse().map_err(|_| Error::NotANumber(s.to_string())))
                .collect();
            // Evaluate the line using the processor.
            recurse(&numbers?, processor)
        })
        .collect();

    Ok(result?.iter().sum())
}

// Evaluates a "level" by calculating the differences and passing it to a processor.
// Returns the difference returned from the process, or 0 if the input is all zeros.
fn recurse(numbers: &Vec<i32>, processor: Processor) -> Result<i32, Error> {
    let is_all_zeros = numbers.iter().find(|&&i| i != 0) == None;
    if is_all_zeros {
        return Ok(0);
    }

    // Using a sliding window, calculate the differences between succeeding numbers
    // and gathers those deltas in a new vector.
    let differences: Vec<i32> = numbers
        .iter()
        .tuple_windows::<(_, _)>()
        .map(|t| t.1 - t.0)
        .collect();

    // Recurse on these vector of deltas.
    let diff = recurse(&differences, processor)?;
    // Feed the original "level" numbers and the difference returned from recursion
    // to the processor.
    processor(numbers, diff)
}

fn part1(input: &str) -> Result<(), Error> {
    let result = parse_and_process(input, |numbers, difference| {
        let num = numbers.last().ok_or(Error::OutOfBounds)?;
        Ok(num + difference)
    })?;
    println!("Part 1: {}", result);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let result = parse_and_process(input, |numbers, difference| {
        let num = numbers.first().ok_or(Error::OutOfBounds)?;
        Ok(num - difference)
    })?;
    println!("Part 2: {}", result);
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
