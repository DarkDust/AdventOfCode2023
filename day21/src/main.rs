use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
enum Error {}

struct Map {
    rocks: HashSet<(isize, isize)>,
    positions: HashSet<(isize, isize)>,
    x_len: isize,
    y_len: isize,
}

impl Map {
    fn new(input: &str) -> Result<Map, Error> {
        let mut rocks: HashSet<(isize, isize)> = HashSet::new();
        let mut positions: HashSet<(isize, isize)> = HashSet::new();
        let mut x_len = 0;
        let mut y_len = 0;

        for (y, line) in input.lines().enumerate() {
            x_len = line.len() as isize;
            y_len += 1;

            for (x, char) in line.chars().enumerate() {
                match char {
                    '#' => _ = rocks.insert((x as isize, y as isize)),
                    'S' => _ = positions.insert((x as isize, y as isize)),
                    _ => (),
                }
            }
        }

        return Ok(Map {
            rocks,
            positions,
            x_len,
            y_len,
        });
    }

    fn step<F>(&self, next_steps: F) -> Map
    where
        F: Fn(&Map, &mut HashSet<(isize, isize)>, &(isize, isize)),
    {
        let mut new_positions: HashSet<(isize, isize)> = HashSet::new();

        for pos in &self.positions {
            next_steps(self, &mut new_positions, pos);
        }

        Map {
            rocks: self.rocks.clone(),
            positions: new_positions,
            x_len: self.x_len,
            y_len: self.y_len,
        }
    }

    fn next_steps_limited(&self, positions: &mut HashSet<(isize, isize)>, from: &(isize, isize)) {
        let x = from.0;
        let y = from.1;

        if x > 0 {
            self.push_step(positions, x - 1, y);
        }
        if x + 1 < self.x_len {
            self.push_step(positions, x + 1, y);
        }
        if y > 0 {
            self.push_step(positions, x, y - 1);
        }
        if y + 1 < self.y_len {
            self.push_step(positions, x, y + 1);
        }
    }

    fn next_steps_infinite(&self, positions: &mut HashSet<(isize, isize)>, from: &(isize, isize)) {
        let x = from.0;
        let y = from.1;

        self.push_step(positions, x - 1, y);
        self.push_step(positions, x + 1, y);
        self.push_step(positions, x, y - 1);
        self.push_step(positions, x, y + 1);
    }

    fn push_step(&self, positions: &mut HashSet<(isize, isize)>, x: isize, y: isize) {
        let normalized_pos = (x.rem_euclid(self.x_len), y.rem_euclid(self.y_len));
        if !self.rocks.contains(&normalized_pos) {
            positions.insert((x, y));
        }
    }
}

fn interpolate(values: Vec<(isize, isize)>, xi: isize) -> isize {
    let mut result: f64 = 0.0;

    for i in 0..values.len() {
        let mut term = values[i].1 as f64;
        for j in 0..values.len() {
            if i == j {
                continue;
            }

            term *= (xi - values[j].0) as f64 / (values[i].0 - values[j].0) as f64;
        }
        result += term
    }

    result as isize
}

fn part1(input: &str) -> Result<(), Error> {
    let mut map = Map::new(input)?;
    for _ in 0..64 {
        map = map.step(Map::next_steps_limited);
    }
    println!("Part 1: {}", map.positions.len());
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut map = Map::new(input)?;
    let x1 = map.x_len / 2;
    let x2 = x1 + map.x_len;
    let x3 = x2 + map.x_len;
    let mut y1 = 0;
    let mut y2 = 0;
    let mut y3 = 0;

    for i in 1..=x3 {
        map = map.step(Map::next_steps_infinite);
        if i == x1 {
            y1 = map.positions.len() as isize;
        } else if i == x2 {
            y2 = map.positions.len() as isize;
        } else if i == x3 {
            y3 = map.positions.len() as isize;
        }
    }

    let values = vec![(x1, y1), (x2, y2), (x3, y3)];
    println!("Part 2: {}", interpolate(values, 26501365));

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
