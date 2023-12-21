use regex::Regex;
use std::{collections::HashMap, ops::Range, time::Instant};

// TODO: Optimize; there's just four parts, can map them to integers and ditch the hash map for the parts.

#[derive(Debug)]
enum Error {
    InvalidRegex,
    InvalidRule,
    InvalidRating,
    MissingWorkflow,
    MissingPartInEvaluation,
    NoWorkflowResult,
}

#[derive(Clone)]
enum Rule {
    LessThan {
        part: String,
        value: usize,
        workflow: String,
    },
    GreaterThan {
        part: String,
        value: usize,
        workflow: String,
    },
    Accept,
    Reject,
    Jump(String),
}

impl Rule {
    fn from(string: &str) -> Option<Rule> {
        match string {
            "A" => return Some(Rule::Accept),
            "R" => return Some(Rule::Reject),
            _ => (),
        }

        let mut parts = string.split(':');
        let rule_or_workflow = parts.next()?;

        if let Some(workflow) = parts.next() {
            if let Some(index) = rule_or_workflow.find("<") {
                let part = &rule_or_workflow[..index];
                let value = &rule_or_workflow[index + 1..].parse().ok()?;
                return Some(Rule::LessThan {
                    part: part.to_string(),
                    value: *value,
                    workflow: workflow.to_string(),
                });
            }

            if let Some(index) = rule_or_workflow.find(">") {
                let part = &rule_or_workflow[..index];
                let value = &rule_or_workflow[index + 1..].parse().ok()?;
                return Some(Rule::GreaterThan {
                    part: part.to_string(),
                    value: *value,
                    workflow: workflow.to_string(),
                });
            }

            return None;
        } else {
            return Some(Rule::Jump(rule_or_workflow.to_string()));
        }
    }
}

#[derive(Clone)]
struct Limits {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl Limits {
    fn less_than(&self, name: &String, value: usize) -> Limits {
        match name.as_str() {
            "x" => Limits {
                x: self.x.start..value,
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            "m" => Limits {
                x: self.x.clone(),
                m: self.m.start..value,
                a: self.a.clone(),
                s: self.s.clone(),
            },
            "a" => Limits {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.start..value,
                s: self.s.clone(),
            },
            "s" => Limits {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.start..value,
            },
            _ => panic!("Unknown part"),
        }
    }

    fn greater_than(&self, name: &String, value: usize) -> Limits {
        match name.as_str() {
            "x" => Limits {
                x: value + 1..self.x.end,
                m: self.m.clone(),
                a: self.a.clone(),
                s: self.s.clone(),
            },
            "m" => Limits {
                x: self.x.clone(),
                m: value + 1..self.m.end,
                a: self.a.clone(),
                s: self.s.clone(),
            },
            "a" => Limits {
                x: self.x.clone(),
                m: self.m.clone(),
                a: value + 1..self.a.end,
                s: self.s.clone(),
            },
            "s" => Limits {
                x: self.x.clone(),
                m: self.m.clone(),
                a: self.a.clone(),
                s: value + 1..self.s.end,
            },
            _ => panic!("Unknown part"),
        }
    }

    fn value(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }
}

struct Evaluator<'a> {
    rules: &'a HashMap<String, Vec<Rule>>,
    parts: HashMap<String, usize>,
}

impl<'a> Evaluator<'a> {
    fn new(rules: &'a HashMap<String, Vec<Rule>>, parts: &Vec<(String, usize)>) -> Evaluator<'a> {
        let mut mapped: HashMap<String, usize> = HashMap::new();
        for (name, value) in parts {
            mapped.insert(name.clone(), *value);
        }

        Evaluator {
            rules,
            parts: mapped,
        }
    }

    fn eval(&self) -> Result<bool, Error> {
        let mut workflow_name = &"in".to_string();

        'outer: while let Some(rule_list) = self.rules.get(workflow_name) {
            for rule in rule_list {
                match rule {
                    Rule::Accept => return Ok(true),
                    Rule::Reject => return Ok(false),
                    Rule::Jump(target) => {
                        workflow_name = target;
                        continue 'outer;
                    }
                    Rule::LessThan {
                        part,
                        value,
                        workflow,
                    } => {
                        let part_value =
                            self.parts.get(part).ok_or(Error::MissingPartInEvaluation)?;
                        if part_value < value {
                            match workflow.as_str() {
                                "A" => return Ok(true),
                                "R" => return Ok(false),
                                _ => (),
                            }
                            workflow_name = workflow;
                            continue 'outer;
                        }
                    }
                    Rule::GreaterThan {
                        part,
                        value,
                        workflow,
                    } => {
                        let part_value =
                            self.parts.get(part).ok_or(Error::MissingPartInEvaluation)?;
                        if part_value > value {
                            match workflow.as_str() {
                                "A" => return Ok(true),
                                "R" => return Ok(false),
                                _ => (),
                            }
                            workflow_name = workflow;
                            continue 'outer;
                        }
                    }
                }
            }

            return Err(Error::NoWorkflowResult);
        }

        Err(Error::MissingWorkflow)
    }

    fn value(&self) -> usize {
        self.parts.iter().fold(0, |acc, entry| acc + entry.1)
    }

    fn find_combinations(&self) -> usize {
        let limits = Limits {
            x: 1..4001,
            m: 1..4001,
            a: 1..4001,
            s: 1..4001,
        };

        self.limit(&"in".to_string(), &limits)
    }

    fn limit(&self, workflow_or_action: &String, current: &Limits) -> usize {
        if workflow_or_action == "A" {
            return current.value();
        }
        if workflow_or_action == "R" {
            return 0;
        }

        let mut current = current.clone();
        let mut sum = 0;

        for rule in self.rules.get(workflow_or_action).unwrap() {
            match rule {
                Rule::Accept => {
                    sum += current.value();
                    return sum;
                }
                Rule::Reject => (),
                Rule::Jump(target) => {
                    sum += self.limit(target, &current);
                }
                Rule::LessThan {
                    part,
                    value,
                    workflow,
                } => {
                    let limited = current.less_than(part, *value);
                    sum += self.limit(workflow, &limited);
                    current = current.greater_than(part, value - 1);
                }
                Rule::GreaterThan {
                    part,
                    value,
                    workflow,
                } => {
                    let limited = current.greater_than(part, *value);
                    sum += self.limit(workflow, &limited);
                    current = current.less_than(part, value + 1);
                }
            }
        }

        sum
    }
}

fn parse(input: &str) -> Result<(HashMap<String, Vec<Rule>>, Vec<Vec<(String, usize)>>), Error> {
    let rule_re = Regex::new(r"^([a-z]+)\{(.*)\}$").map_err(|_| Error::InvalidRegex)?;

    let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();
    let mut ratings: Vec<Vec<(String, usize)>> = Vec::new();

    let mut is_rating = false;
    for line in input.lines() {
        if line.is_empty() {
            is_rating = true;
            continue;
        }

        if is_rating {
            let rating: Result<Vec<(String, usize)>, Error> = line[1..line.len() - 1]
                .split(",")
                .map(|raw_rating| {
                    let mut parts = raw_rating.split("=");
                    let name = parts.next().ok_or(Error::InvalidRating)?;
                    let rating: usize = parts
                        .next()
                        .ok_or(Error::InvalidRating)?
                        .parse()
                        .map_err(|_| Error::InvalidRating)?;
                    Ok((name.to_string(), rating))
                })
                .collect();
            ratings.push(rating?);
        } else {
            let captures = rule_re.captures(line).ok_or(Error::InvalidRegex)?;
            let name = captures.get(1).unwrap().as_str();
            let rule_list = captures.get(2).unwrap().as_str().split(",");
            let parsed: Result<Vec<Rule>, Error> = rule_list
                .map(|s| Rule::from(s).ok_or(Error::InvalidRule))
                .collect();
            rules.insert(name.to_string(), parsed?);
        }
    }

    Ok((rules, ratings))
}

fn part1(input: &str) -> Result<(), Error> {
    let (rules, ratings) = parse(input)?;
    let mut accepted = 0;

    for rating in ratings {
        let evaluator = Evaluator::new(&rules, &rating);
        if evaluator.eval()? {
            accepted += evaluator.value();
        }
    }

    println!("Part 1: {}", accepted);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let (rules, _) = parse(input)?;
    let evaluator = Evaluator::new(&rules, &Vec::new());
    println!("Part 2: {}", evaluator.find_combinations());
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
