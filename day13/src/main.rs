use std::time::Instant;

#[derive(Debug)]
enum Error {
    FailedToDesmudge,
}

struct Map {
    mirrors: Vec<Vec<bool>>,
    is_transposed: bool,
    did_fix_smudge: bool,
}

impl Map {
    fn new(mirrors: Vec<Vec<bool>>) -> Map {
        Map {
            mirrors,
            is_transposed: false,
            did_fix_smudge: false,
        }
    }

    fn transpose(&self) -> Map {
        let rows = self.mirrors.len();
        let columns = self.mirrors[0].len();
        let mut transposed: Vec<Vec<bool>> =
            (0..columns).map(|_| Vec::with_capacity(rows)).collect();
        for row in &self.mirrors {
            for (x, value) in row.iter().enumerate() {
                transposed[x].push(*value);
            }
        }

        Map {
            mirrors: transposed,
            is_transposed: !self.is_transposed,
            did_fix_smudge: self.did_fix_smudge,
        }
    }

    fn row_reflects(&self, source: usize, target: usize) -> bool {
        self.mirrors[source] == self.mirrors[target]
    }

    fn check_reflection(&self, index: usize, delta: usize) -> usize {
        let target = index + delta;
        if self.row_reflects(index, target) {
            if index > 0 && (target + 1) < self.mirrors.len() {
                self.check_reflection(index - 1, delta + 2)
            } else {
                (delta + 1) / 2
            }
        } else {
            0
        }
    }

    fn desmudge_check(&self, index: usize, delta: usize, avoid_score: usize) -> Option<usize> {
        let target = index + delta;
        if self.row_reflects(index, target) {
            if index > 0 && (target + 1) < self.mirrors.len() {
                return self.desmudge_check(index - 1, delta + 2, avoid_score);
            } else {
                return None;
            }
        } else if let Some(map) = self.fix_smudge(index, target) {
            let score = map.score(avoid_score);
            if score > 0 {
                return Some(score);
            }
        }
        None
    }

    fn fix_smudge(&self, row1: usize, row2: usize) -> Option<Map> {
        if self.did_fix_smudge {
            return None;
        }

        let diff: Vec<bool> = self.mirrors[row1]
            .iter()
            .zip(self.mirrors[row2].iter())
            .map(|t| t.0 ^ t.1)
            .collect();
        let diff_count: usize = diff.iter().map(|b| if *b { 1 } else { 0 }).sum();
        if diff_count != 1 {
            return None;
        }

        let mut patched = self.mirrors.clone();
        patched[row1] = self.mirrors[row1]
            .iter()
            .zip(diff.iter())
            .map(|t| t.0 ^ t.1)
            .collect();

        Some(Map {
            mirrors: patched,
            is_transposed: self.is_transposed,
            did_fix_smudge: true,
        })
    }

    fn find_perfect_reflection(&self, avoid_score: usize) -> usize {
        let num_mirrors = self.mirrors.len();
        for i in 0..num_mirrors - 1 {
            let len = self.check_reflection(i, 1);
            if len == 0 {
                continue;
            }

            if (i + len) == num_mirrors - 1 || (i + 1) == len {
                let count = i + 1;
                if self.calc_score(count) != avoid_score {
                    return count;
                }
            }
        }
        0
    }

    fn calc_score(&self, count: usize) -> usize {
        if self.is_transposed {
            count
        } else {
            count * 100
        }
    }

    fn score(&self, avoid_score: usize) -> usize {
        let count = self.find_perfect_reflection(avoid_score);
        if count > 0 {
            return self.calc_score(count);
        }

        let transposed = self.transpose();
        let transposed_count = transposed.find_perfect_reflection(avoid_score);
        return transposed.calc_score(transposed_count);
    }

    fn desmudged_score(&self) -> Option<usize> {
        let score = self.score(0);
        for i in 0..self.mirrors.len() - 1 {
            if let Some(result) = self.desmudge_check(i, 1, score) {
                return Some(result);
            }
        }

        if !self.is_transposed {
            return self.transpose().desmudged_score();
        }

        None
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let mut acc: Vec<Vec<bool>> = Vec::new();
    let mut result = 0;

    for line in input.lines() {
        if line.is_empty() {
            if !acc.is_empty() {
                let map = Map::new(acc);
                acc = Vec::new();

                let score = map.score(0);
                assert!(score != 0);
                result += score;
            }
            continue;
        }

        let row = line.chars().map(|c| c == '#').collect();
        acc.push(row);
    }

    if !acc.is_empty() {
        let map = Map::new(acc);
        let score = map.score(0);
        // assert!(score != 0);
        result += score;
    }

    println!("Part 1: {}", result);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut acc: Vec<Vec<bool>> = Vec::new();
    let mut result = 0;

    for line in input.lines() {
        if line.is_empty() {
            if !acc.is_empty() {
                let map = Map::new(acc);
                acc = Vec::new();

                result += map.desmudged_score().ok_or(Error::FailedToDesmudge)?;
            }
            continue;
        }

        let row = line.chars().map(|c| c == '#').collect();
        acc.push(row);
    }

    if !acc.is_empty() {
        let map = Map::new(acc);
        result += map.desmudged_score().ok_or(Error::FailedToDesmudge)?;
    }

    println!("Part 2: {}", result);
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
