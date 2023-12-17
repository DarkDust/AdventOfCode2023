use std::{
    collections::{BinaryHeap, HashMap},
    num::ParseIntError,
    time::Instant,
};

#[derive(Debug)]
enum Error {
    ParseError(ParseIntError),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

struct HeatLossMap {
    temperatures: Vec<Vec<usize>>,
    count_x: usize,
    count_y: usize,
    start: (usize, usize),
    target: (usize, usize),
}

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
struct Crucible {
    pos: (usize, usize),
    dir: Direction,
    dir_steps: usize,
}

#[derive(PartialEq, Eq)]
struct HeapEntry {
    crucible: Crucible,
    f_score: usize,
}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score).reverse()
    }
}

impl HeatLossMap {
    fn new(input: &str) -> Result<HeatLossMap, Error> {
        let maybe_temperatures: Result<Vec<Vec<usize>>, Error> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| String::from(c).parse().map_err(|e| Error::ParseError(e)))
                    .collect()
            })
            .collect();
        let temperatures = maybe_temperatures?;

        let count_x = temperatures[0].len();
        let count_y = temperatures.len();

        Ok(HeatLossMap {
            temperatures,
            count_x,
            count_y,
            start: (0, 0),
            target: (count_x - 1, count_y - 1),
        })
    }

    // A* path finding algorithm, slightly adapted for this puzzle: no need for "external" f_score,
    // and no need for "came_from" as we're not interested in the actual path; the cost is all we
    // want.
    fn a_star<F>(&self, advance: F) -> usize
    where
        F: Fn(&HeatLossMap, &Crucible, Direction, &mut Vec<Crucible>),
    {
        let start_crucible1 = Crucible {
            pos: self.start,
            dir: Direction::East,
            dir_steps: 0,
        };
        let start_entry1 = HeapEntry {
            crucible: start_crucible1,
            f_score: self.estimate_cost(&start_crucible1),
        };
        let start_crucible2 = Crucible {
            pos: self.start,
            dir: Direction::South,
            dir_steps: 0,
        };
        let start_entry2 = HeapEntry {
            crucible: start_crucible2,
            f_score: self.estimate_cost(&start_crucible2),
        };

        let mut open_set = BinaryHeap::new();
        open_set.push(start_entry1);
        open_set.push(start_entry2);

        let mut g_score = HashMap::new();
        g_score.insert(start_crucible1, 0);
        g_score.insert(start_crucible2, 0);

        while let Some(current_entry) = open_set.pop() {
            let current = current_entry.crucible;
            if current.pos == self.target {
                // Usually, A* would reconstruct the path here. We're not interested in that,
                // just need the cost.
                return *g_score.get(&current).unwrap();
            }

            // It's a critical error if there's no g_score for current.
            let current_g_score = *g_score.get(&current).unwrap();
            for candidate in self.possible_moves(&current, &advance) {
                let tentative_g_score =
                    current_g_score + self.temperatures[candidate.pos.1][candidate.pos.0];
                if tentative_g_score < *g_score.get(&candidate).unwrap_or(&usize::MAX) {
                    g_score.insert(candidate, tentative_g_score);
                    open_set.push(HeapEntry {
                        crucible: candidate,
                        f_score: tentative_g_score + self.estimate_cost(&candidate),
                    });
                }
            }
        }

        panic!("Did not find a path");
    }

    fn estimate_cost(&self, crucible: &Crucible) -> usize {
        let from = crucible.pos;
        let to = self.target;
        (from.0.max(to.0) - from.0.min(to.0)) + (from.1.max(to.1) - from.1.min(to.1))
    }

    fn possible_moves<F>(&self, crucible: &Crucible, advance: F) -> Vec<Crucible>
    where
        F: Fn(&HeatLossMap, &Crucible, Direction, &mut Vec<Crucible>),
    {
        let mut result = Vec::new();

        advance(self, crucible, crucible.dir, &mut result);
        match crucible.dir {
            Direction::North | Direction::South => {
                advance(self, crucible, Direction::East, &mut result);
                advance(self, crucible, Direction::West, &mut result);
            }
            Direction::East | Direction::West => {
                advance(self, crucible, Direction::North, &mut result);
                advance(self, crucible, Direction::South, &mut result);
            }
        }

        result
    }

    fn advance_crucible_part1(
        &self,
        crucible: &Crucible,
        dir: Direction,
        result: &mut Vec<Crucible>,
    ) {
        if dir == crucible.dir && crucible.dir_steps >= 2 {
            return;
        }

        if let Some(new_pos) = self.advance_pos(crucible.pos, dir) {
            let new_steps = if dir == crucible.dir {
                crucible.dir_steps + 1
            } else {
                0
            };
            result.push(Crucible {
                pos: new_pos,
                dir: dir,
                dir_steps: new_steps,
            })
        }
    }

    fn advance_crucible_part2(
        &self,
        crucible: &Crucible,
        dir: Direction,
        result: &mut Vec<Crucible>,
    ) {
        if dir == crucible.dir && crucible.dir_steps >= 9 {
            return;
        }

        if dir != crucible.dir && crucible.dir_steps < 3 {
            return;
        }

        if let Some(new_pos) = self.advance_pos(crucible.pos, dir) {
            let new_steps = if dir == crucible.dir {
                crucible.dir_steps + 1
            } else {
                0
            };

            if new_pos == self.target && new_steps < 4 {
                return;
            }

            result.push(Crucible {
                pos: new_pos,
                dir: dir,
                dir_steps: new_steps,
            })
        }
    }

    fn advance_pos(&self, pos: (usize, usize), dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::North => {
                if pos.1 > 0 {
                    Some((pos.0, pos.1 - 1))
                } else {
                    None
                }
            }
            Direction::South => {
                if (pos.1 + 1) < self.count_y {
                    Some((pos.0, pos.1 + 1))
                } else {
                    None
                }
            }
            Direction::West => {
                if pos.0 > 0 {
                    Some((pos.0 - 1, pos.1))
                } else {
                    None
                }
            }
            Direction::East => {
                if (pos.0 + 1) < self.count_x {
                    Some((pos.0 + 1, pos.1))
                } else {
                    None
                }
            }
        }
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let map = HeatLossMap::new(input)?;
    println!(
        "Part 1: {}",
        map.a_star(HeatLossMap::advance_crucible_part1)
    );
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let map = HeatLossMap::new(input)?;
    println!(
        "Part 2: {}",
        map.a_star(HeatLossMap::advance_crucible_part2)
    );
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
