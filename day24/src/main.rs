use rug::Float;
use std::time::Instant;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Clone, PartialEq)]
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

impl std::ops::Sub for &Coord3 {
    type Output = Coord3;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
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

    fn _intersection(
        p1: (f64, f64),
        p2: (f64, f64),
        p3: (f64, f64),
        p4: (f64, f64),
    ) -> Option<(f64, f64)> {
        // I seem to be missing something about how to use Rug, the following code is very verbose
        // and involves a lot of (probably unnecessary) clones… and it's slow.
        let prec = 128;
        let x1 = Float::with_val(prec, p1.0);
        let x2 = Float::with_val(prec, p2.0);
        let x3 = Float::with_val(prec, p3.0);
        let x4 = Float::with_val(prec, p4.0);
        let y1 = Float::with_val(prec, p1.1);
        let y2 = Float::with_val(prec, p2.1);
        let y3 = Float::with_val(prec, p3.1);
        let y4 = Float::with_val(prec, p4.1);

        // https://en.wikipedia.org/wiki/Line–line_intersection#Given_two_points_on_each_line
        let denom = (x1.clone() - x2.clone()) * (y3.clone() - y4.clone())
            - (y1.clone() - y2.clone()) * (x3.clone() - x4.clone());
        if denom.is_zero() {
            return None;
        }

        let cross_12 = x1.clone() * y2.clone() - y1.clone() * x2.clone();
        let cross_34 = x3.clone() * y4.clone() - y3.clone() * x4.clone();

        let x_nom = cross_12.clone() * (x3.clone() - x4.clone())
            - (x1.clone() - x2.clone()) * cross_34.clone();
        let y_nom = cross_12.clone() * (y3.clone() - y4.clone())
            - (y1.clone() - y2.clone()) * cross_34.clone();
        Some(((x_nom / denom.clone()).to_f64(), (y_nom / denom).to_f64()))
    }

    fn intersection_xy(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let p1 = &self.pos;
        let p2 = &self.pos + &self.vel;
        let p3 = &other.pos;
        let p4 = &other.pos + &other.vel;
        Self::_intersection((p1.x, p1.y), (p2.x, p2.y), (p3.x, p3.y), (p4.x, p4.y))
    }

    fn intersection_xz(&self, other: &Hailstone) -> Option<(f64, f64)> {
        let p1 = &self.pos;
        let p2 = &self.pos + &self.vel;
        let p3 = &other.pos;
        let p4 = &other.pos + &other.vel;
        Self::_intersection((p1.x, p1.z), (p2.x, p2.z), (p3.x, p3.z), (p4.x, p4.z))
    }

    fn time_to_intersection(&self, pos: (f64, f64)) -> f64 {
        let delta_x = pos.0 - self.pos.x;
        delta_x / self.vel.x
    }

    fn change_velocity(&self, vel: &Coord3) -> Hailstone {
        Hailstone {
            pos: self.pos.clone(),
            vel: &self.vel - vel,
        }
    }
}

fn all_intersect_xy(hailstones: &Vec<Hailstone>) -> Option<(f64, f64)> {
    let empty: (f64, f64) = (0.0, 0.0);
    let mut intersection = empty.clone();

    for i in 0..hailstones.len() {
        let h1 = &hailstones[i];

        for j in i + 1..hailstones.len() {
            let h2 = &hailstones[j];

            if let Some(intersection_xy) = h1.intersection_xy(h2) {
                if intersection == empty {
                    intersection = (intersection_xy.0, intersection_xy.1);
                } else if intersection != intersection_xy {
                    return None;
                }
            } else {
                // Parallel lines in xy plane. Ignore.
            }
        }
    }

    Some((intersection.0 as f64, intersection.1 as f64))
}

fn all_intersect_xz(hailstones: &Vec<Hailstone>) -> Option<(f64, f64)> {
    let empty = (0.0, 0.0);
    let mut intersection = empty.clone();

    for i in 0..hailstones.len() {
        let h1 = &hailstones[i];

        for j in i + 1..hailstones.len() {
            let h2 = &hailstones[j];

            if let Some(intersection_xz) = h1.intersection_xz(h2) {
                if intersection == empty {
                    intersection = intersection_xz;
                } else if intersection != intersection_xz {
                    return None;
                }
            } else {
                // Parallel lines in xy plane. Ignore.
            }
        }
    }

    Some(intersection)
}

fn search_stone_parameters(
    hailstones: &Vec<Hailstone>,
    search_range: std::ops::Range<isize>,
) -> Option<Hailstone> {
    for x in search_range.start..=search_range.end {
        for y in search_range.start..=search_range.end {
            let mut vel = Coord3 {
                x: x as f64,
                y: y as f64,
                z: 0.0,
            };
            let transposed: Vec<_> = hailstones.iter().map(|h| h.change_velocity(&vel)).collect();
            if let Some(xy) = all_intersect_xy(&transposed) {
                println!("Have first intersection at {}, {}", xy.0, xy.1);
                for z in search_range.start..=search_range.end {
                    vel.z = z as f64;
                    let transposed: Vec<_> =
                        hailstones.iter().map(|h| h.change_velocity(&vel)).collect();
                    if let Some(xz) = all_intersect_xz(&transposed) {
                        return Some(Hailstone {
                            pos: Coord3 {
                                x: xy.0,
                                y: xy.1,
                                z: xz.1,
                            },
                            vel,
                        });
                    }
                }
            }
        }
    }

    None
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

            if let Some(intersection) = h1.intersection_xy(h2) {
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
    let maybe_hailstones: Result<Vec<Hailstone>, Error> =
        input.lines().map(|line| Hailstone::new(line)).collect();
    let hailstones = maybe_hailstones?;

    if let Some(stone) = search_stone_parameters(&hailstones, -500..500) {
        println!("Stone: {} -> {}", stone.pos, stone.vel);
        println!(
            "Part 2: {}",
            stone.pos.x.round() + stone.pos.y.round() + stone.pos.z.round()
        );
    } else {
        println!("Part 2: no result")
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
