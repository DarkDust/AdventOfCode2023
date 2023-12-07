use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::time::Instant;

#[derive(Hash, PartialEq, Eq)]
struct Coord(i32, i32);

#[derive(Hash, PartialEq, Eq, Clone)]
struct EntryLong {
    x1: i32,
    x2: i32,
    y: i32,
    num: i32,
}

fn surrounding(coord: &Coord, number: i32) -> Vec<Coord> {
    let digits: i32 = (number as f64).log10().floor() as i32;
    let mut result = Vec::new();

    for y in coord.1 - 1..=coord.1 + 1 {
        for x in coord.0 - 1..=coord.0 + digits + 1 {
            result.push(Coord(x, y));
        }
    }

    return result;
}

fn has_symbol_neighbour(coord: &Coord, number: i32, symbols: &HashMap<Coord, char>) -> bool {
    for neighbour in surrounding(coord, number) {
        if symbols.contains_key(&neighbour) {
            return true;
        }
    }

    return false;
}

fn insert_long(numbers: &mut HashMap<Coord, EntryLong>, entry: EntryLong) {
    for x in entry.x1..=entry.x2 {
        let new_entry = entry.clone();
        numbers.insert(Coord(x, new_entry.y), new_entry);
    }
}

fn gear_ratio(coord: &Coord, numbers: &HashMap<Coord, EntryLong>) -> i32 {
    let mut candidates: HashSet<EntryLong> = HashSet::new();

    for neighbour in surrounding(coord, 1) {
        if let Some(entry) = numbers.get(&neighbour) {
            candidates.insert(entry.clone());
        }
    }

    if candidates.len() != 2 {
        return 0;
    }

    return candidates.iter().fold(1, |a, e| a * e.num);
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut symbols: HashMap<Coord, char> = HashMap::new();
    let mut numbers: HashMap<Coord, i32> = HashMap::new();

    let mut y = 0;
    for line in input.lines() {
        let mut start = -1;
        let mut num = 0;
        for (x, char) in line.char_indices() {
            if let Some(digit) = char.to_digit(10) {
                if start < 0 {
                    start = x as i32;
                }
                num = num * 10 + (digit as i32);
                continue;
            }

            if start >= 0 {
                numbers.insert(Coord(start, y), num);
                num = 0;
                start = -1;
            }

            if char != '.' {
                symbols.insert(Coord(x as i32, y), char);
            }
        }

        if start >= 0 {
            numbers.insert(Coord(start, y), num);
        }

        y += 1;
    }

    let valid = numbers
        .iter()
        .filter(|(k, v)| has_symbol_neighbour(k, **v, &symbols));
    let sum: i32 = valid.map(|(_, v)| v).fold(0, |a, v| a + v);

    println!("Part 1: {sum}");
    return Ok(());
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut symbols: HashMap<Coord, char> = HashMap::new();
    let mut numbers: HashMap<Coord, EntryLong> = HashMap::new();

    let mut y = 0;
    for line in input.lines() {
        let mut start = -1;
        let mut num = 0;
        for (x, char) in line.char_indices() {
            if let Some(digit) = char.to_digit(10) {
                if start < 0 {
                    start = x as i32;
                }
                num = num * 10 + (digit as i32);
                continue;
            }

            if start >= 0 {
                let entry = EntryLong {
                    x1: start,
                    x2: (x - 1) as i32,
                    y: y,
                    num: num,
                };
                insert_long(&mut numbers, entry);
                num = 0;
                start = -1;
            }

            if char != '.' {
                symbols.insert(Coord(x as i32, y), char);
            }
        }

        if start >= 0 {
            let entry = EntryLong {
                x1: start,
                x2: (line.len() - 1) as i32,
                y: y,
                num: num,
            };
            insert_long(&mut numbers, entry);
        }

        y += 1;
    }

    let ratios = symbols
        .iter()
        .filter(|(_, char)| **char == '*')
        .map(|(coord, _)| gear_ratio(coord, &numbers));
    let sum = ratios.fold(0, |a, r| a + r);

    println!("Part 2: {sum}");
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
