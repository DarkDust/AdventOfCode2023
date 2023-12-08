use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug)]
enum Error {
    MissingInstruction,
    ParseError,
    RegexError(regex::Error),
    WalkError,
    MissingNode,
    MalformedInstruction,
}

fn walk(nodes: &HashMap<&str, (&str, &str)>, instructions: &str) -> Result<usize, Error> {
    let mut steps = 0;
    let mut instructions = instructions.chars().cycle();
    let mut id = "AAA";

    while id != "ZZZ" {
        let direction = instructions.next().ok_or(Error::WalkError)?;
        let (left, right) = nodes.get(id).ok_or(Error::MissingNode)?;
        match direction {
            'L' => id = *left,
            'R' => id = *right,
            _ => return Err(Error::MalformedInstruction),
        }

        steps += 1;
    }

    return Ok(steps);
}

fn walk_ghost(
    nodes: &HashMap<&str, (&str, &str)>,
    instructions: &str,
    start: &str,
) -> Result<usize, Error> {
    let mut steps = 0;
    let mut instructions = instructions.chars().cycle();
    let mut id = start;

    while !id.ends_with("Z") {
        let direction = instructions.next().ok_or(Error::WalkError)?;
        let (left, right) = nodes.get(id).ok_or(Error::MissingNode)?;
        match direction {
            'L' => id = *left,
            'R' => id = *right,
            _ => return Err(Error::MalformedInstruction),
        }

        steps += 1;
    }

    return Ok(steps);
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}

fn lcm(a: usize, b: usize) -> usize {
    return a * (b / gcd(a, b));
}

fn walk_ghosts(nodes: &HashMap<&str, (&str, &str)>, instructions: &str) -> Result<usize, Error> {
    let start_ids: Vec<&str> = nodes.keys().filter(|k| k.ends_with("A")).cloned().collect();
    let steps: Result<HashSet<usize>, Error> = start_ids
        .iter()
        .map(|start| walk_ghost(nodes, instructions, start))
        .collect();

    println!("{:?}", steps);

    let foo = steps?.iter().cloned().reduce(|acc, step| lcm(acc, step));
    foo.ok_or(Error::WalkError)
}

fn part1(input: &str) -> Result<(), Error> {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut lines = input.lines();
    let instructions = lines.next().ok_or(Error::MissingInstruction)?;
    let re =
        Regex::new(r"^(\S+)\s*=\s*\((\S+)\s*,\s*(\S+)\)$").map_err(|e| Error::RegexError(e))?;
    for line in lines.skip(1) {
        let matches = re.captures(line).ok_or(Error::ParseError)?;
        let id = matches.get(1).ok_or(Error::ParseError)?.as_str();
        let left = matches.get(2).ok_or(Error::ParseError)?.as_str();
        let right = matches.get(3).ok_or(Error::ParseError)?.as_str();
        nodes.insert(id, (left, right));
    }

    let steps = walk(&nodes, instructions)?;

    println!("Part 1: {steps}");
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut nodes: HashMap<&str, (&str, &str)> = HashMap::new();

    let mut lines = input.lines();
    let instructions = lines.next().ok_or(Error::MissingInstruction)?;
    let re =
        Regex::new(r"^(\S+)\s*=\s*\((\S+)\s*,\s*(\S+)\)$").map_err(|e| Error::RegexError(e))?;
    for line in lines.skip(1) {
        let matches = re.captures(line).ok_or(Error::ParseError)?;
        let id = matches.get(1).ok_or(Error::ParseError)?.as_str();
        let left = matches.get(2).ok_or(Error::ParseError)?.as_str();
        let right = matches.get(3).ok_or(Error::ParseError)?.as_str();
        nodes.insert(id, (left, right));
    }

    let steps = walk_ghosts(&nodes, instructions)?;

    println!("Part 2: {steps}");
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
