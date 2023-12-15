use std::time::Instant;

#[derive(Debug)]
enum Error {
    InvalidInstruction,
    InvalidBoxNumber,
}

#[derive(Clone)]
struct Lens {
    label: String,
    focal: usize,
}

struct Lenses {
    boxes: Vec<Vec<Lens>>,
}

fn hash(input: &str) -> usize {
    input
        .chars()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

impl Lenses {
    fn new() -> Lenses {
        Lenses {
            boxes: vec![Vec::new(); 256],
        }
    }

    fn process(&mut self, input: &str) -> Result<(), Error> {
        if let Some(label) = input.strip_suffix('-') {
            let box_nr = hash(label);
            let b = self.boxes.get_mut(box_nr).ok_or(Error::InvalidBoxNumber)?;
            if let Some(index) = b.iter().position(|l| l.label == label) {
                b.remove(index);
            }
            return Ok(());
        }

        let mut parts = input.split('=');
        let label = parts.next().ok_or(Error::InvalidInstruction)?;
        let focal = parts
            .next()
            .map(|s| s.parse::<usize>())
            .ok_or(Error::InvalidInstruction)?
            .map_err(|_| Error::InvalidInstruction)?;

        let lens = Lens {
            label: label.to_string(),
            focal,
        };
        let box_nr = hash(label);
        let b = self.boxes.get_mut(box_nr).ok_or(Error::InvalidBoxNumber)?;
        if let Some(index) = b.iter().position(|l| l.label == label) {
            b[index] = lens;
        } else {
            b.push(lens);
        }

        Ok(())
    }

    fn focusing_power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(box_nr, b)| {
                b.iter()
                    .enumerate()
                    .map(|(lens_nr, lens)| (1 + box_nr) * (lens_nr + 1) * lens.focal)
                    .sum::<usize>()
            })
            .sum()
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let sum: usize = input
        .lines()
        .map(|line| line.split(',').map(|part| hash(part)).sum::<usize>())
        .sum();
    println!("Part 1: {sum}");
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut lenses = Lenses::new();
    for line in input.lines() {
        for instruction in line.split(',') {
            lenses.process(instruction)?;
        }
    }

    println!("Part 2: {}", lenses.focusing_power());
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
