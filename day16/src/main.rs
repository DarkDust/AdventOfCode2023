use crate::Direction::{East, North, South, West};
use std::{collections::HashSet, time::Instant};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Direction {
    North,
    West,
    South,
    East,
}

enum Field {
    Empty,
    MirrorSlash,
    MirrorBackslash,
    SplitterVertical,
    SplitterHorizontal,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Beam {
    pos: (usize, usize),
    dir: Direction,
}

struct Contraption {
    fields: Vec<Vec<Field>>,
    count_x: usize,
    count_y: usize,
    energized: HashSet<(usize, usize)>,
    cycle_detector: HashSet<Beam>,
    beams: Vec<Beam>,
}

impl Contraption {
    fn new(input: &str) -> Result<Contraption, Error> {
        let maybe_fields: Result<Vec<Vec<Field>>, Error> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => Ok(Field::Empty),
                        '/' => Ok(Field::MirrorSlash),
                        '\\' => Ok(Field::MirrorBackslash),
                        '|' => Ok(Field::SplitterVertical),
                        '-' => Ok(Field::SplitterHorizontal),
                        _ => Err(Error::InvalidInput),
                    })
                    .collect()
            })
            .collect();
        let fields = maybe_fields?;
        let count_x = fields[0].len();
        let count_y = fields.len();
        Ok(Contraption {
            fields,
            count_x,
            count_y,
            energized: HashSet::new(),
            cycle_detector: HashSet::new(),
            beams: Vec::new(),
        })
    }

    fn beam_step(&mut self, beam: &Beam) -> Vec<Beam> {
        if !self.cycle_detector.insert(beam.clone()) {
            // Seen the same position and direction again, there must be a kind of cycle.
            return Vec::new();
        }
        self.energized.insert(beam.pos);

        match self.fields[beam.pos.1][beam.pos.0] {
            Field::Empty => return self.advance_beam(beam, &beam.dir),
            Field::MirrorSlash => match beam.dir {
                North => return self.advance_beam(beam, &East),
                West => return self.advance_beam(beam, &South),
                South => return self.advance_beam(beam, &West),
                East => return self.advance_beam(beam, &North),
            },
            Field::MirrorBackslash => match beam.dir {
                North => return self.advance_beam(beam, &West),
                West => return self.advance_beam(beam, &North),
                South => return self.advance_beam(beam, &East),
                East => return self.advance_beam(beam, &South),
            },
            Field::SplitterHorizontal => match beam.dir {
                East | West => return self.advance_beam(beam, &beam.dir),
                North | South => return self.split_beam(beam, East, West),
            },
            Field::SplitterVertical => match beam.dir {
                North | South => return self.advance_beam(beam, &beam.dir),
                East | West => return self.split_beam(beam, North, South),
            },
        }
    }

    fn next_pos(&self, pos: (usize, usize), dir: &Direction) -> Option<(usize, usize)> {
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

    fn advance_beam(&self, beam: &Beam, dir: &Direction) -> Vec<Beam> {
        if let Some(next_pos) = self.next_pos(beam.pos, &dir) {
            vec![Beam {
                pos: next_pos,
                dir: dir.clone(),
            }]
        } else {
            vec![]
        }
    }

    fn split_beam(&self, beam: &Beam, dir1: Direction, dir2: Direction) -> Vec<Beam> {
        let mut result = Vec::new();
        if let Some(next_pos) = self.next_pos(beam.pos, &dir1) {
            result.push(Beam {
                pos: next_pos,
                dir: dir1,
            });
        }
        if let Some(next_pos) = self.next_pos(beam.pos, &dir2) {
            result.push(Beam {
                pos: next_pos,
                dir: dir2,
            });
        }
        result
    }

    fn trace_from(&mut self, pos: (usize, usize), dir: Direction) -> usize {
        self.beams = vec![Beam { pos, dir }];
        self.cycle_detector.clear();
        self.energized.clear();

        while !self.beams.is_empty() {
            let old_beams: Vec<_> = self.beams.drain(..).collect();
            for beam in old_beams {
                let mut advanced = self.beam_step(&beam);
                self.beams.append(&mut advanced);
            }
        }

        self.energized.len()
    }

    fn trace_beams_from_all_sides(&mut self) -> usize {
        let mut max_energized = 0;

        for x in 0..self.count_x {
            max_energized = max_energized.max(self.trace_from((x, 0), South));
            max_energized = max_energized.max(self.trace_from((x, self.count_y - 1), North));
        }
        for y in 0..self.count_y {
            max_energized = max_energized.max(self.trace_from((0, y), East));
            max_energized = max_energized.max(self.trace_from((self.count_x - 1, y), West));
        }

        max_energized
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let mut contraption = Contraption::new(input)?;
    println!("Part 1: {}", contraption.trace_from((0, 0), East));
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut contraption = Contraption::new(input)?;
    println!("Part 2: {}", contraption.trace_beams_from_all_sides());
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
