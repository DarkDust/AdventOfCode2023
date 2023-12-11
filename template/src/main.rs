use std::time::Instant;

#[derive(Debug)]
enum Error {}

fn part1(input: &str) -> Result<(), Error> {
    println!("Part 1: TBD");
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    println!("Part 2: TBD");
    return Ok(());
}

fn main() -> Result<(), Error> {
    let input = include_str!("../rsc/sample1.txt");

    let start1 = Instant::now();
    part1(input)?;
    println!("Elapsed: {:.2?}\n", start1.elapsed());

    let start2 = Instant::now();
    part2(input)?;
    println!("Elapsed: {:.2?}", start2.elapsed());

    Ok(())
}
