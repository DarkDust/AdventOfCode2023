use std::{collections::HashSet, time::Instant, usize};

#[derive(Debug)]
enum Error {
    InvalidFieldPattern,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Field {
    Empty,
    Cube,
    Round,
}

impl Field {
    fn as_string(&self) -> &'static str {
        match self {
            Field::Empty => ".",
            Field::Cube => "#",
            Field::Round => "O",
        }
    }
}

struct Map {
    fields: Vec<Field>,
    count_x: usize,
    count_y: usize,
}

impl Map {
    fn new(fields: Vec<Vec<Field>>) -> Map {
        let count_y = fields.len();
        let count_x = fields[0].len();
        Map {
            fields: fields.into_iter().flatten().collect(),
            count_x,
            count_y,
        }
    }

    fn dump(&self) {
        for y in 0..self.count_y {
            let line: String = (0..self.count_x)
                .map(|x| self.get(x, y).as_string())
                .collect();
            println!("{}", line);
        }

        println!();
    }

    #[inline]
    fn get(&self, x: usize, y: usize) -> &Field {
        &self.fields[y * self.count_x + x]
    }

    #[inline]
    fn set(&mut self, x: usize, y: usize, f: Field) {
        self.fields[y * self.count_x + x] = f;
    }

    fn tilt_north(&mut self) {
        for y in 0..self.count_y {
            for x in 0..self.count_x {
                self.move_north(x, y);
            }
        }
    }

    fn tilt_south(&mut self) {
        for y in (0..self.count_y).rev() {
            for x in 0..self.count_x {
                self.move_south(x, y);
            }
        }
    }

    fn tilt_west(&mut self) {
        for y in 0..self.count_y {
            for x in 0..self.count_x {
                self.move_west(x, y);
            }
        }
    }

    fn tilt_east(&mut self) {
        for y in 0..self.count_y {
            for x in (0..self.count_x).rev() {
                self.move_east(x, y);
            }
        }
    }

    fn move_north(&mut self, x: usize, y: usize) {
        let mut ly = y;
        while ly > 0 && self.get(x, ly) == &Field::Round && self.get(x, ly - 1) == &Field::Empty {
            self.set(x, ly - 1, Field::Round);
            self.set(x, ly, Field::Empty);
            ly -= 1;
        }
    }

    fn move_south(&mut self, x: usize, y: usize) {
        let mut ly = y;
        let max_y = self.count_y - 1;
        while ly < max_y && self.get(x, ly) == &Field::Round && self.get(x, ly + 1) == &Field::Empty
        {
            self.set(x, ly + 1, Field::Round);
            self.set(x, ly, Field::Empty);
            ly += 1;
        }
    }

    fn move_west(&mut self, x: usize, y: usize) {
        let mut lx = x;
        while lx > 0 && self.get(lx, y) == &Field::Round && self.get(lx - 1, y) == &Field::Empty {
            self.set(lx - 1, y, Field::Round);
            self.set(lx, y, Field::Empty);
            lx -= 1;
        }
    }

    fn move_east(&mut self, x: usize, y: usize) {
        let mut lx = x;
        let max_x = self.count_x - 1;
        while lx < max_x && self.get(lx, y) == &Field::Round && self.get(lx + 1, y) == &Field::Empty
        {
            self.set(lx + 1, y, Field::Round);
            self.set(lx, y, Field::Empty);
            lx += 1;
        }
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn cache(&self, cycle_cache: &mut HashSet<Vec<Field>>) -> bool {
        cycle_cache.insert(self.fields.clone())
    }

    fn load_north(&self) -> usize {
        let mut sum = 0;

        for y in 0..self.count_y {
            for x in 0..self.count_x {
                if self.get(x, y) == &Field::Round {
                    sum += self.count_y - y;
                }
            }
        }

        sum
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let fields: Result<Vec<Vec<Field>>, Error> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Ok(Field::Empty),
                    '#' => Ok(Field::Cube),
                    'O' => Ok(Field::Round),
                    _ => Err(Error::InvalidFieldPattern),
                })
                .collect()
        })
        .collect();

    let mut map = Map::new(fields?);
    map.tilt_north();

    println!("Part 1: {}", map.load_north());
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let fields: Result<Vec<Vec<Field>>, Error> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Ok(Field::Empty),
                    '#' => Ok(Field::Cube),
                    'O' => Ok(Field::Round),
                    _ => Err(Error::InvalidFieldPattern),
                })
                .collect()
        })
        .collect();

    let mut map = Map::new(fields?);
    let mut cycle_cache = HashSet::new();
    let mut cycle_start = 0;

    let mut i = 0;
    let repetitions = 1000000000;
    while i < repetitions {
        map.cycle();
        if !map.cache(&mut cycle_cache) {
            // Seen a constellation again.
            if cycle_start == 0 {
                // Seen it again for the first time. From here on, we know there's a cycle.
                cycle_start = i;
                cycle_cache.clear();
                map.cache(&mut cycle_cache);
                println!("Found cycle at {i}");
            } else {
                // Seen the cycle repeat again. Now we know its length.
                let cycle_len = i - cycle_start;
                println!("Found cycle end at {i}, length {cycle_len}");
                cycle_cache.clear();

                // Skip all the remaining full cycles, do the the last partial cycle.
                // (Integer division rounds down.)
                i += ((repetitions - i) / cycle_len) * cycle_len;
            }
        }

        i += 1;
    }

    println!("Part 2: {}", map.load_north());
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
