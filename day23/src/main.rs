use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
    time::Instant,
};

#[derive(Debug)]
enum Error {
    InvalidField,
    InvalidMap,
    NoPathFound,
}

#[derive(PartialEq, Eq)]
enum Field {
    Wall,
    Empty,
    SlopeNorth,
    SlopeWest,
    SlopeSouth,
    SlopeEast,
}

#[derive(PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

struct Map {
    fields: Vec<Field>,
    x_len: usize,
    y_len: usize,
    start: (usize, usize),
    target: (usize, usize),
}

struct PathFinder {
    map: Map,
    slopes_are_slippery: bool,
    max_distances: HashMap<(usize, usize), usize>,
}

impl Map {
    fn new(input: &str) -> Result<Map, Error> {
        let mut fields = Vec::new();
        let mut x_len = 0;
        let mut y_len = 0;

        for line in input.lines() {
            x_len = line.len();
            y_len += 1;

            let line_fields: Result<Vec<Field>, Error> = line
                .chars()
                .map(|c| match c {
                    '#' => Ok(Field::Wall),
                    '.' => Ok(Field::Empty),
                    '^' => Ok(Field::SlopeNorth),
                    '<' => Ok(Field::SlopeWest),
                    'v' => Ok(Field::SlopeSouth),
                    '>' => Ok(Field::SlopeEast),
                    _ => Err(Error::InvalidField),
                })
                .collect();
            fields.append(&mut line_fields?);
        }

        let start = (1, 0);
        let target = (x_len - 2, y_len - 1);
        if fields[(start.1 * x_len) + start.0] != Field::Empty {
            return Err(Error::InvalidMap);
        }
        if fields[(target.1 * x_len) + target.0] != Field::Empty {
            return Err(Error::InvalidMap);
        }

        Ok(Map {
            fields,
            x_len,
            y_len,
            start,
            target,
        })
    }

    #[inline]
    fn pos(&self, x: usize, y: usize) -> usize {
        (y * self.x_len) + x
    }

    fn field(&self, pos: (usize, usize)) -> &Field {
        &self.fields[self.pos(pos.0, pos.1)]
    }

    fn dump(&self, visited: &HashSet<(usize, usize)>) {
        for y in 0..self.y_len {
            for x in 0..self.x_len {
                if visited.contains(&(x, y)) {
                    print!("O");
                } else {
                    match self.field((x, y)) {
                        Field::Wall => print!("#"),
                        Field::Empty => print!("."),
                        Field::SlopeNorth => print!("^"),
                        Field::SlopeWest => print!("<"),
                        Field::SlopeSouth => print!("v"),
                        Field::SlopeEast => print!(">"),
                    }
                }
            }
            println!();
        }
        println!();
    }
}

impl PathFinder {
    fn new(map: Map, slopes_are_slippery: bool) -> PathFinder {
        PathFinder {
            map,
            slopes_are_slippery,
            max_distances: HashMap::new(),
        }
    }

    fn find_longest(&mut self) -> Option<usize> {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        self.walk(self.map.start, &mut visited)
    }

    fn walk(
        &mut self,
        from: (usize, usize),
        visited: &mut HashSet<(usize, usize)>,
    ) -> Option<usize> {
        let mut current_pos = from;
        loop {
            let movements = self.possible_movements(current_pos, visited);
            match movements.len() {
                0 => return None, // Dead end
                1 => {
                    current_pos = movements[0];
                    self.record_movement(current_pos, visited);

                    if current_pos == self.map.target {
                        return Some(visited.len());
                    }
                }
                _ => {
                    let mut max_dist = 0;
                    for pos in movements {
                        let mut forked_visited = visited.clone();
                        self.record_movement(pos, &mut forked_visited);

                        if let Some(path_length) = self.walk(pos, &mut forked_visited) {
                            max_dist = max_dist.max(path_length);
                        }
                    }
                    if max_dist == 0 {
                        return None;
                    } else {
                        return Some(max_dist);
                    }
                }
            }
        }
    }

    fn possible_movements(
        &self,
        from: (usize, usize),
        visited: &HashSet<(usize, usize)>,
    ) -> Vec<(usize, usize)> {
        let mut result = Vec::new();

        if let Some(to) = self.can_move(from, Direction::North, visited) {
            result.push(to);
        }
        if let Some(to) = self.can_move(from, Direction::West, visited) {
            result.push(to);
        }
        if let Some(to) = self.can_move(from, Direction::South, visited) {
            result.push(to);
        }
        if let Some(to) = self.can_move(from, Direction::East, visited) {
            result.push(to);
        }

        return result;
    }

    fn can_move(
        &self,
        from: (usize, usize),
        dir: Direction,
        visited: &HashSet<(usize, usize)>,
    ) -> Option<(usize, usize)> {
        let to: (usize, usize) = match dir {
            Direction::North => {
                if from.1 == 0 {
                    return None;
                }
                (from.0, from.1 - 1)
            }
            Direction::West => {
                if from.0 == 0 {
                    return None;
                }
                (from.0 - 1, from.1)
            }
            Direction::South => {
                if from.1 + 1 >= self.map.y_len {
                    return None;
                }
                (from.0, from.1 + 1)
            }
            Direction::East => {
                if from.0 + 1 >= self.map.x_len {
                    return None;
                }
                (from.0 + 1, from.1)
            }
        };

        if visited.contains(&to) {
            return None;
        }

        if self.slopes_are_slippery {
            match (self.map.field(to), dir) {
                (Field::Wall, _) => return None,
                (Field::Empty, _) => (),
                (Field::SlopeNorth, Direction::North) => (),
                (Field::SlopeWest, Direction::West) => (),
                (Field::SlopeSouth, Direction::South) => (),
                (Field::SlopeEast, Direction::East) => (),
                _ => return None,
            }
        } else if self.map.field(to) == &Field::Wall {
            return None;
        }

        if let Some(known_distance) = self.max_distances.get(&to) {
            if visited.len() + 1 < *known_distance {
                // There's already a path that's known to be longer.
                return None;
            }
        }

        Some(to)
    }

    fn record_movement(&mut self, pos: (usize, usize), visited: &mut HashSet<(usize, usize)>) {
        visited.insert(pos);
        self.max_distances.insert(pos, visited.len());
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let map = Map::new(input)?;
    let mut path_finder = PathFinder::new(map, true);
    let max_distance = path_finder.find_longest().ok_or(Error::NoPathFound)?;
    println!("Part 1: {}", max_distance);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let map = Map::new(input)?;
    let mut path_finder = PathFinder::new(map, false);
    let max_distance = path_finder.find_longest().ok_or(Error::NoPathFound)?;
    println!("Part 2: {}", max_distance);
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
