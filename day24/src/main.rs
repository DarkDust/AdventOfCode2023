use std::time::Instant;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Clone)]
struct Coord3 {
    x: f64,
    y: f64,
    z: f64,
}

struct Hailstone {
    pos: Coord3,
    vel: Coord3,
}

impl Coord3 {
    fn new(input: &str) -> Result<Coord3, Error> {
        let mut parts = input.split(",").map(|s| s.trim());
        let x = parts
            .next()
            .ok_or(Error::InvalidInput)?
            .parse()
            .map_err(|_| Error::InvalidInput)?;
        let y = parts
            .next()
            .ok_or(Error::InvalidInput)?
            .parse()
            .map_err(|_| Error::InvalidInput)?;
        let z = parts
            .next()
            .ok_or(Error::InvalidInput)?
            .parse()
            .map_err(|_| Error::InvalidInput)?;
        Ok(Coord3 { x, y, z })
    }

    fn scale(&self, factor: f64) -> Coord3 {
        Coord3 {
            x: self.x * factor,
            y: self.y * factor,
            z: self.z * factor,
        }
    }
}

impl std::ops::Add for &Coord3 {
    type Output = Coord3;

    fn add(self, rhs: Self) -> Self::Output {
        Coord3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::fmt::Display for Coord3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {}", self.x, self.y, self.z)
    }
}

impl Hailstone {
    fn new(line: &str) -> Result<Hailstone, Error> {
        let mut parts = line.split("@");
        let pos = Coord3::new(parts.next().ok_or(Error::InvalidInput)?)?;
        let vel = Coord3::new(parts.next().ok_or(Error::InvalidInput)?)?;
        Ok(Hailstone { pos, vel })
    }

    fn intersection_flat(&self, other: &Hailstone) -> Option<(f64, f64)> {
        // I don't understand why, but if I used `&self.pos + &self.vel` for p2 (and same for p4),
        // the result were too few intersections in the real input data. I suspect rounding errors.
        let p1 = &self.pos;
        let p2 = &self.pos + &self.vel.scale(10.0);
        let p3 = &other.pos;
        let p4 = &other.pos + &other.vel.scale(10.0);

        // https://en.wikipedia.org/wiki/Line–line_intersection#Given_two_points_on_each_line
        // Can likely be writter in a more readable way… 🤷‍♂️
        let x_nom = (p1.x * p2.y - p1.y * p2.x) * (p3.x - p4.x)
            - (p1.x - p2.x) * (p3.x * p4.y - p3.y * p4.x);
        let x_denom = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
        let y_nom = (p1.x * p2.y - p1.y * p2.x) * (p3.y - p4.y)
            - (p1.y - p2.y) * (p3.x * p4.y - p3.y * p4.x);
        let y_denom = (p1.x - p2.x) * (p3.y - p4.y) - (p1.y - p2.y) * (p3.x - p4.x);
        if x_denom == 0.0 || y_denom == 0.0 {
            return None;
        }
        Some((x_nom / x_denom, y_nom / y_denom))
    }

    fn time_to_intersection(&self, pos: (f64, f64)) -> f64 {
        let delta_x = pos.0 - self.pos.x;
        delta_x / self.vel.x
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let maybe_hailstones: Result<Vec<Hailstone>, Error> =
        input.lines().map(|line| Hailstone::new(line)).collect();
    let hailstones = maybe_hailstones?;

    fn is_inside(i: &(f64, f64)) -> bool {
        let bounds_min: f64 = 200000000000000.0;
        let bounds_max: f64 = 400000000000000.0;
        // let bounds_min: f64 = 7.0;
        // let bounds_max: f64 = 27.0;

        i.0 >= bounds_min && i.0 <= bounds_max && i.1 >= bounds_min && i.1 <= bounds_max
    }

    let mut count = 0;

    for i in 0..hailstones.len() {
        let h1 = &hailstones[i];

        for j in i + 1..hailstones.len() {
            let h2 = &hailstones[j];

            if let Some(intersection) = h1.intersection_flat(h2) {
                if is_inside(&intersection)
                    && h1.time_to_intersection(intersection) >= 0.0
                    && h2.time_to_intersection(intersection) >= 0.0
                {
                    count += 1;
                }
            }
        }
    }

    println!("Part 1: {}", count);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    println!("Part 2: TBD");
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
