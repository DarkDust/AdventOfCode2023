use std::{collections::HashMap, time::Instant};

#[derive(Debug)]
enum Error {
    InvalidLine,
    InvalidCondition,
    InvalidMatch,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Condition {
    Good,
    Damaged,
    Unknown,
}

struct Row {
    springs: Vec<Condition>,
    matches: Vec<usize>,
    spring_count: usize,
    match_count: usize,
}

struct RowCursor<'a> {
    row: &'a Row,
    spring_i: usize,
    match_i: usize,
}

impl Row {
    fn new(line: &str) -> Result<Row, Error> {
        let mut parts = line.split_whitespace();
        let conditions = parts.next().ok_or(Error::InvalidLine)?;
        let list = parts.next().ok_or(Error::InvalidLine)?;
        if parts.next() != None {
            return Err(Error::InvalidLine);
        }

        let springs: Result<Vec<_>, Error> = conditions
            .chars()
            .map(|c| match c {
                '.' => Ok(Condition::Good),
                '#' => Ok(Condition::Damaged),
                '?' => Ok(Condition::Unknown),
                _ => Err(Error::InvalidCondition),
            })
            .collect();

        let matches: Result<Vec<usize>, Error> = list
            .split(',')
            .map(|s| s.parse().map_err(|_| Error::InvalidMatch))
            .collect();

        let springs = springs?;
        let spring_count = springs.len();
        let matches = matches?;
        let match_count = matches.len();
        Ok(Row {
            springs,
            matches,
            spring_count,
            match_count,
        })
    }

    fn unfold(&mut self) {
        let mut unfolded_springs = Vec::new();
        unfolded_springs.append(&mut self.springs.clone());
        unfolded_springs.push(Condition::Unknown);
        unfolded_springs.append(&mut self.springs.clone());
        unfolded_springs.push(Condition::Unknown);
        unfolded_springs.append(&mut self.springs.clone());
        unfolded_springs.push(Condition::Unknown);
        unfolded_springs.append(&mut self.springs.clone());
        unfolded_springs.push(Condition::Unknown);
        unfolded_springs.append(&mut self.springs.clone());

        self.springs = unfolded_springs;
        self.spring_count = self.springs.len();
        self.matches = self.matches.repeat(5);
        self.match_count = self.matches.len();
    }

    fn start(&self) -> RowCursor {
        return RowCursor {
            row: self,
            spring_i: 0,
            match_i: 0,
        };
    }
}

impl RowCursor<'_> {
    // Whether there are any spring conditions left to examine.
    fn is_at_spring_end(&self) -> bool {
        self.spring_i == self.row.spring_count
    }

    // Whether there are any matches left.
    fn is_at_match_end(&self) -> bool {
        self.match_i == self.row.match_count
    }

    // Whether the current match can apply at the current position.
    fn can_match(&self) -> bool {
        if self.is_at_match_end() {
            return false;
        }

        let match_len = self.row.matches[self.match_i];
        let available = self.row.spring_count - self.spring_i;
        if match_len > available {
            return false;
        }

        for i in self.spring_i..self.spring_i + match_len {
            match self.row.springs[i] {
                Condition::Good => return false,
                Condition::Damaged => (),
                Condition::Unknown => (),
            }
        }

        if match_len == available {
            // End of condition list.
            return true;
        }

        // Not the end. A good or unknown spring must follow.
        match self.row.springs[self.spring_i + match_len] {
            Condition::Good => true,
            Condition::Damaged => false,
            Condition::Unknown => true,
        }
    }

    // Whether no more matches are left, and the spring conditions are already at the end or the
    // remaining ones are all good.
    fn can_finish(&self) -> bool {
        if !self.is_at_match_end() {
            return false;
        }

        for i in self.spring_i..self.row.spring_count {
            match self.row.springs[i] {
                Condition::Good => (),
                Condition::Damaged => return false,
                Condition::Unknown => (),
            }
        }
        true
    }

    // Skip one spring.
    fn skip(&self) -> RowCursor {
        RowCursor {
            row: self.row,
            spring_i: self.spring_i + 1,
            match_i: self.match_i,
        }
    }

    // Skip all good springs starting at the current position (which must be "good").
    // Returns None if the end of the spring conditions list is reached.
    fn skip_good(&self) -> Option<RowCursor> {
        for i in self.spring_i..self.row.spring_count {
            if self.row.springs[i] == Condition::Good {
                continue;
            }

            return Some(RowCursor {
                row: self.row,
                spring_i: i,
                match_i: self.match_i,
            });
        }

        None
    }

    // Apply the current match and advance to the next match.
    fn consume(&self) -> RowCursor {
        let match_len = self.row.matches[self.match_i];
        let available = self.row.spring_count - self.spring_i;
        assert!(match_len <= available);

        let skip = if match_len == available {
            match_len
        } else {
            match_len + 1
        };

        RowCursor {
            row: self.row,
            spring_i: self.spring_i + skip,
            match_i: self.match_i + 1,
        }
    }

    // Lookup a count for the receiver in the cache.
    fn lookup_cache<'a, 'b>(
        &'a self,
        cache: &'b HashMap<(usize, usize), usize>,
    ) -> Option<&'b usize> {
        let key = (self.spring_i, self.match_i);
        cache.get(&key)
    }

    // Store the count in the cache.
    fn store_cache(&self, cache: &mut HashMap<(usize, usize), usize>, count: usize) {
        let key = (self.spring_i, self.match_i);
        cache.insert(key, count);
    }

    // Recursively count all possible iterations.
    fn count(&self, cache: &mut HashMap<(usize, usize), usize>) -> usize {
        if self.can_finish() {
            return 1;
        }
        if self.is_at_spring_end() {
            return 0;
        }
        if let Some(count) = self.lookup_cache(cache) {
            return *count;
        }

        match self.row.springs[self.spring_i] {
            Condition::Good => self.skip_good().map(|r| r.count(cache)).unwrap_or(0),
            Condition::Damaged => {
                let count = if self.can_match() {
                    self.consume().count(cache)
                } else {
                    0
                };
                self.store_cache(cache, count);
                return count;
            }
            Condition::Unknown => {
                let count = if self.can_match() {
                    self.consume().count(cache) + self.skip().count(cache)
                } else {
                    self.skip().count(cache)
                };
                self.store_cache(cache, count);
                return count;
            }
        }
    }
}

fn part1(input: &str) -> Result<(), Error> {
    let mut sum = 0;
    for line in input.lines() {
        let row = Row::new(line)?;
        let mut cache = HashMap::new();
        sum += row.start().count(&mut cache);
    }
    println!("Part 1: {sum}");
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut sum = 0;
    for line in input.lines() {
        let mut row = Row::new(line)?;
        row.unfold();
        let mut cache = HashMap::new();
        sum += row.start().count(&mut cache);
    }
    println!("Part 2: {sum}");
    return Ok(());
}

fn main() -> Result<(), Error> {
    let input = include_str!("../rsc/input.txt");

    let start1 = Instant::now();
    part1(input)?;
    println!("Elapsed: {:.2?}\n", start1.elapsed());

    // The main trick for part 2 is the cache. Without the cache, the code did not find a solution within 8 hours.
    // With the cache, the solution was found in 52ms (!).
    let start2 = Instant::now();
    part2(input)?;
    println!("Elapsed: {:.2?}", start2.elapsed());

    Ok(())
}
