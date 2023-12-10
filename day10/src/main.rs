use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
enum Error {
    MissingStart,
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North = 1 << 0,
    East = 1 << 1,
    South = 1 << 2,
    West = 1 << 3,
    Start = 1 << 4,
}

trait ContainsDirection {
    fn contains_dir(&self, dir: Direction) -> bool;
}

impl ContainsDirection for i32 {
    fn contains_dir(&self, dir: Direction) -> bool {
        (self & dir as i32) != 0
    }
}

fn parse_tile(char: char) -> i32 {
    match char {
        'S' => Direction::Start as i32,
        '|' => Direction::North as i32 | Direction::South as i32,
        '-' => Direction::East as i32 | Direction::West as i32,
        'L' => Direction::North as i32 | Direction::East as i32,
        'J' => Direction::North as i32 | Direction::West as i32,
        '7' => Direction::South as i32 | Direction::West as i32,
        'F' => Direction::South as i32 | Direction::East as i32,
        _ => 0,
    }
}

fn directions(tile: i32) -> Vec<Direction> {
    let mut directions = Vec::new();

    if tile.contains_dir(Direction::North) {
        directions.push(Direction::North);
    }
    if tile.contains_dir(Direction::East) {
        directions.push(Direction::East);
    }
    if tile.contains_dir(Direction::South) {
        directions.push(Direction::South);
    }
    if tile.contains_dir(Direction::West) {
        directions.push(Direction::West);
    }

    return directions;
}

struct Field {
    field: Vec<Vec<i32>>,
    count_x: usize,
    count_y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize,
}

impl Field {
    fn new(field: Vec<Vec<i32>>) -> Field {
        let count_y = field.len();
        let count_x = field.first().unwrap_or(&Vec::new()).len();
        Field {
            field,
            count_x,
            count_y,
        }
    }

    fn get(&self, coord: Coord) -> i32 {
        match self.field.get(coord.y) {
            Some(row) => *row.get(coord.x).unwrap_or(&0),
            None => 0,
        }
    }

    fn find_start(&self) -> Option<Coord> {
        for (y, row) in self.field.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile.contains_dir(Direction::Start) {
                    return Some(Coord { x, y });
                }
            }
        }

        return None;
    }

    fn start_candidates(&self, coord: Coord) -> Vec<Coord> {
        let mut coords = Vec::new();

        if coord.x > 0 {
            let candidate = Coord {
                x: coord.x - 1,
                y: coord.y,
            };
            let tile = self.get(candidate);
            if tile.contains_dir(Direction::East) {
                coords.push(candidate);
            }
        }
        if coord.x < self.count_x {
            let candidate = Coord {
                x: coord.x + 1,
                y: coord.y,
            };
            let tile = self.get(candidate);
            if tile.contains_dir(Direction::West) {
                coords.push(candidate);
            }
        }
        if coord.y > 0 {
            let candidate = Coord {
                x: coord.x,
                y: coord.y - 1,
            };
            let tile = self.get(candidate);
            if tile.contains_dir(Direction::South) {
                coords.push(candidate);
            }
        }
        if coord.y < self.count_y {
            let candidate = Coord {
                x: coord.x,
                y: coord.y + 1,
            };
            let tile = self.get(candidate);
            if tile.contains_dir(Direction::North) {
                coords.push(candidate);
            }
        }

        return coords;
    }

    fn next_coords(&self, coord: Coord, tile: i32) -> Vec<Coord> {
        let mut coords = Vec::new();

        for direction in directions(tile) {
            if direction == Direction::North && coord.y > 0 {
                coords.push(Coord {
                    x: coord.x,
                    y: coord.y - 1,
                });
            } else if direction == Direction::South && coord.y < self.count_y {
                coords.push(Coord {
                    x: coord.x,
                    y: coord.y + 1,
                });
            } else if direction == Direction::West && coord.x > 0 {
                coords.push(Coord {
                    x: coord.x - 1,
                    y: coord.y,
                });
            } else if direction == Direction::East && coord.x < self.count_x {
                coords.push(Coord {
                    x: coord.x + 1,
                    y: coord.y,
                });
            }
        }

        return coords;
    }

    fn find_path(&self, coord: Coord, from: Coord, path: &mut Vec<Coord>) -> bool {
        let tile = self.get(coord);
        if tile.contains_dir(Direction::Start) {
            path.push(coord);
            return true;
        }

        for next in self.next_coords(coord, tile) {
            if next == from {
                continue;
            }

            if self.find_path(next, coord, path) {
                path.push(coord);
                return true;
            }
        }

        return false;
    }

    // Clear all tiles that are not part of the path.
    fn clear_non_path(&mut self, path: &Vec<Coord>) {
        let lookup: HashSet<&Coord> = path.iter().collect();

        for (y, row) in self.field.iter_mut().enumerate() {
            for x in 0..row.len() {
                if !lookup.contains(&Coord { x, y }) {
                    row[x] = 0;
                }
            }
        }
    }

    // Count all tiles that are inside the path (clear_non_path must have been called).
    fn count_inside(&self) -> usize {
        // Simple raycasting, count the intersections. Only need to consider tiles with a north or
        // south connection (pick one, use only that one). Avoids issues with horizontal path tiles.
        let mut sum = 0;

        for row in self.field.iter() {
            let mut is_inside = false;

            for tile in row {
                if tile.contains_dir(Direction::South) {
                    is_inside = !is_inside;
                } else if *tile == 0 && is_inside {
                    sum += 1;
                }
            }
        }

        sum
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let mut rows = Vec::new();

    for line in input.lines() {
        let row = line.chars().map(parse_tile).collect();
        rows.push(row);
    }

    let field = Field::new(rows);
    let start = field.find_start().ok_or(Error::MissingStart)?;
    let candidates = field.start_candidates(start);
    for candidate in candidates {
        let mut path = Vec::new();
        field.find_path(candidate, start, &mut path);
        if path.is_empty() {
            continue;
        }
        println!("Part 1: {}", (path.len() + 1) / 2);
        break;
    }

    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut rows = Vec::new();

    for line in input.lines() {
        let row = line.chars().map(parse_tile).collect();
        rows.push(row);
    }

    let mut field = Field::new(rows);
    let start = field.find_start().ok_or(Error::MissingStart)?;
    let candidates = field.start_candidates(start);
    for candidate in candidates {
        let mut path = Vec::new();
        field.find_path(candidate, start, &mut path);
        if path.is_empty() {
            continue;
        }

        field.clear_non_path(&path);

        println!("Part 2: {}", field.count_inside());
        break;
    }

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
