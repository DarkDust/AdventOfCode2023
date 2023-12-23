use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

#[derive(Debug)]
enum Error {
    ParseError,
}

#[derive(PartialEq, Clone)]
struct Coord {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(PartialEq, Clone)]
struct Brick {
    id: usize,
    start: Coord,
    end: Coord,
}

impl Coord {
    fn new(input: &str) -> Result<Coord, Error> {
        let mut parts = input.split(",");
        let x: usize = parts
            .next()
            .ok_or(Error::ParseError)?
            .parse()
            .map_err(|_| Error::ParseError)?;
        let y: usize = parts
            .next()
            .ok_or(Error::ParseError)?
            .parse()
            .map_err(|_| Error::ParseError)?;
        let z: usize = parts
            .next()
            .ok_or(Error::ParseError)?
            .parse()
            .map_err(|_| Error::ParseError)?;
        Ok(Coord { x, y, z })
    }
}

impl Brick {
    fn new(line: &str) -> Result<Brick, Error> {
        static mut COUNTER: usize = 0;

        let mut parts = line.split("~");
        let c1 = Coord::new(parts.next().ok_or(Error::ParseError)?)?;
        let c2 = Coord::new(parts.next().ok_or(Error::ParseError)?)?;

        let id = unsafe {
            COUNTER += 1;
            COUNTER
        };

        if c1.y <= c2.y {
            Ok(Brick {
                id,
                start: c1,
                end: c2,
            })
        } else {
            Ok(Brick {
                id,
                start: c2,
                end: c1,
            })
        }
    }

    fn lies_on(&self, other: &Brick) -> bool {
        if self.start.z != other.end.z + 1 {
            return false;
        }

        Self::overlaps(self.start.x, self.end.x, other.start.x, other.end.x)
            && Self::overlaps(self.start.y, self.end.y, other.start.y, other.end.y)
    }

    fn overlaps(b1s: usize, b1e: usize, b2s: usize, b2e: usize) -> bool {
        if b1s < b2s {
            return b1e >= b2s;
        }
        return b2e >= b1s;
    }

    fn move_down(&mut self) {
        self.start.z -= 1;
        self.end.z -= 1;
    }
}

fn settle(bricks: &mut Vec<Brick>) -> usize {
    // Sort by z-order first.
    bricks.sort_by(|b1, b2| b1.start.z.cmp(&b2.start.z));

    let mut settled = Vec::new();
    let mut unsettled = VecDeque::new();
    // How many settled bricks to inspect for filtering, max. No need to iterate over the whole vec.
    let candidate_window: usize = 100;

    for brick in bricks {
        if brick.start.z == 1 {
            settled.push(brick);
        } else {
            unsettled.push_back(brick);
        }
    }

    let mut moved = HashSet::new();
    'outer: while let Some(unsettled_brick) = unsettled.pop_front() {
        let z = unsettled_brick.start.z;
        let settled_len = settled.len();
        let skip_len = if settled_len < candidate_window {
            0
        } else {
            settled_len - candidate_window
        };
        let candidates: Vec<_> = settled
            .iter()
            .skip(skip_len)
            .filter(|b| b.end.z + 1 == z)
            .collect();
        for candidate in candidates {
            if unsettled_brick.lies_on(candidate) {
                settled.push(unsettled_brick);
                continue 'outer;
            }
        }

        unsettled_brick.move_down();
        moved.insert(unsettled_brick.id);

        if unsettled_brick.start.z == 1 {
            // Was moved down, now on ground.
            settled.push(unsettled_brick);
        } else {
            // Need re-evaluate.
            unsettled.push_front(unsettled_brick);
        }
    }

    moved.len()
}

fn count_disintegratable(bricks: &Vec<Brick>) -> usize {
    let mut count = 0;
    for brick in bricks {
        if can_disintegrate(bricks, brick) {
            count += 1;
        }
    }
    count
}

fn can_disintegrate(bricks: &Vec<Brick>, brick: &Brick) -> bool {
    let is_supporting: Vec<_> = bricks
        .iter()
        .filter(|candidate| candidate.lies_on(brick))
        .collect();
    if is_supporting.is_empty() {
        return true;
    }

    let z = brick.end.z;
    let support_candidates: Vec<_> = bricks
        .iter()
        .filter(|b| b.end.z == z && *b != brick)
        .collect();
    if support_candidates.is_empty() {
        return false;
    }

    for supported in is_supporting {
        if support_candidates.iter().position(|b| supported.lies_on(b)) == None {
            return false;
        }
    }

    true
}

fn count_chainreactions(bricks: &Vec<Brick>) -> usize {
    let mut total = 0;

    for i in 0..bricks.len() {
        let mut temp = bricks.clone();
        temp.remove(i);

        total += settle(&mut temp);
    }

    total
}

fn part1(input: &str) -> Result<(), Error> {
    let mut bricks = Vec::new();

    for line in input.lines() {
        let brick = Brick::new(line)?;
        bricks.push(brick);
    }

    settle(&mut bricks);

    println!("Part 1: {}", count_disintegratable(&bricks));
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut bricks = Vec::new();

    for line in input.lines() {
        let brick = Brick::new(line)?;
        bricks.push(brick);
    }

    settle(&mut bricks);

    println!("Part 2: {}", count_chainreactions(&bricks));
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
