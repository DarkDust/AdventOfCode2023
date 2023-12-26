use rand::prelude::*;
use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

#[derive(Debug)]
enum Error {
    InvalidInput,
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Connection {
    component1: usize,
    component2: usize,
}

impl Connection {
    fn is_connected(&self, other: &Connection) -> bool {
        self.component1 == other.component1
            || self.component1 == other.component2
            || self.component2 == other.component1
            || self.component2 == other.component2
    }
}

fn add_component(name: &str, components: &mut Vec<String>) -> usize {
    if let Some(index) = components.iter().position(|n| n == name) {
        return index;
    }

    components.push(name.to_string());
    components.len() - 1
}

fn add_connection(i1: usize, i2: usize, connections: &mut Vec<Connection>) {
    if i1 < i2 {
        connections.push(Connection {
            component1: i1,
            component2: i2,
        })
    } else {
        connections.push(Connection {
            component1: i2,
            component2: i1,
        })
    }
}

fn count_groups(connections: &Vec<Connection>) -> Vec<usize> {
    let mut connections_to_check: HashSet<_> = connections.iter().collect();
    let mut group_lengths = Vec::new();

    while let Some(first) = connections_to_check.iter().next() {
        let mut next: VecDeque<&Connection> = VecDeque::new();
        next.push_back(*first);

        let mut group: HashSet<usize> = HashSet::new();
        while let Some(connection) = next.pop_front() {
            connections_to_check.remove(connection);

            group.insert(connection.component1);
            group.insert(connection.component2);

            for candidate in connections_to_check
                .iter()
                .filter(|c| c.is_connected(connection))
            {
                next.push_back(*candidate);
            }
        }

        group_lengths.push(group.len());
    }

    group_lengths
}

fn remove_connection(
    components: &Vec<String>,
    connections: &mut Vec<Connection>,
    c1: &str,
    c2: &str,
) -> Option<()> {
    let i1 = components.iter().position(|n| n == c1)?;
    let i2 = components.iter().position(|n| n == c2)?;
    let s1 = i1.min(i2);
    let s2 = i1.max(i2);
    let index = connections
        .iter()
        .position(|c| c.component1 == s1 && c.component2 == s2)?;

    connections.remove(index);
    Some(())
}

#[derive(Clone, PartialEq, Eq)]
struct MergedConnection {
    component1: usize,
    component2: usize,
    orig_component1: usize,
    orig_component2: usize,
}

impl MergedConnection {
    fn new(connection: &Connection) -> MergedConnection {
        MergedConnection {
            component1: connection.component1,
            component2: connection.component2,
            orig_component1: connection.component1,
            orig_component2: connection.component2,
        }
    }

    fn to_connection(&self) -> Connection {
        Connection {
            component1: self.orig_component1,
            component2: self.orig_component2,
        }
    }

    fn replace(&self, victim: usize, survivor: usize) -> MergedConnection {
        if self.component1 == victim {
            MergedConnection {
                component1: survivor,
                component2: self.component2,
                orig_component1: self.orig_component1,
                orig_component2: self.orig_component2,
            }
        } else if self.component2 == victim {
            MergedConnection {
                component1: self.component1,
                component2: survivor,
                orig_component1: self.orig_component1,
                orig_component2: self.orig_component2,
            }
        } else {
            panic!("Invalid replacement!")
        }
    }
}

// Part of Karger's Algorithm: contract a connection. Here, the simply remove the connection and
// reroute all connections that target one of the nodes (the victim) to the other (the survivor).
// The victim is removed from the list of components.
fn contract(
    connection: &MergedConnection,
    components: &mut HashSet<usize>,
    connections: &mut Vec<MergedConnection>,
) {
    let survivor = connection.component1;
    let victim = connection.component2;

    components.remove(&victim);
    if let Some(index) = connections.iter().position(|c| c == connection) {
        connections.remove(index);
    } else {
        panic!("Connection not in list!")
    }

    // Need to patch all connections involving the victim. If replacing the connection would
    // result in a connection from the survivor to the survivor, the connection needs to be
    // removed.
    let mut need_replacement = Vec::new();
    let mut self_referencing = Vec::new();
    for (index, candidate) in connections
        .iter()
        .enumerate()
        .filter(|(_, c)| c.component1 == victim || c.component2 == victim)
    {
        if candidate.component1 == survivor || candidate.component2 == survivor {
            self_referencing.push(index)
        } else {
            need_replacement.push(index);
        }
    }

    for index in need_replacement {
        let replacement = connections[index].replace(victim, survivor);
        connections[index] = replacement;
    }

    self_referencing.sort();
    self_referencing.reverse();
    for index in self_referencing {
        connections.remove(index);
    }
}

// Find cuts using Karger's Algorithm.
fn find_cuts(components: &Vec<String>, connections: &Vec<Connection>) -> Vec<Connection> {
    let mut remaining_connections: Vec<MergedConnection> = connections
        .iter()
        .map(|c| MergedConnection::new(c))
        .collect();
    let mut remaining_components: HashSet<usize> = (0..components.len()).collect();

    let mut rng = rand::thread_rng();
    while remaining_components.len() > 2 {
        let connection = remaining_connections.choose(&mut rng).unwrap().clone();
        contract(
            &connection,
            &mut remaining_components,
            &mut remaining_connections,
        );
    }

    remaining_connections
        .iter()
        .map(|c| c.to_connection())
        .collect()
}

fn part1(input: &str) -> Result<(), Error> {
    let mut components: Vec<String> = Vec::new();
    let mut connections: Vec<Connection> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let c1 = parts.next().ok_or(Error::InvalidInput)?;
        let i1 = add_component(c1, &mut components);

        for other in parts.next().ok_or(Error::InvalidInput)?.split_whitespace() {
            let i2 = add_component(other, &mut components);
            add_connection(i1, i2, &mut connections);
        }
    }

    remove_connection(&components, &mut connections, "xhg", "ljl");
    remove_connection(&components, &mut connections, "lkm", "ffj");
    remove_connection(&components, &mut connections, "vgs", "xjb");

    let group_lengths = count_groups(&connections);
    let result = group_lengths.iter().fold(1, |acc, len| acc * len);
    println!("Part 1: {}", result);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Error> {
    let mut components: Vec<String> = Vec::new();
    let mut connections: Vec<Connection> = Vec::new();

    for line in input.lines() {
        let mut parts = line.split(": ");
        let c1 = parts.next().ok_or(Error::InvalidInput)?;
        let i1 = add_component(c1, &mut components);

        for other in parts.next().ok_or(Error::InvalidInput)?.split_whitespace() {
            let i2 = add_component(other, &mut components);
            add_connection(i1, i2, &mut connections);
        }
    }

    loop {
        // Karger's Algorithm is random, it does not always find the optimal solution. We know the
        // optimal cut has three connections, so apply the algorithm until a cut with just three
        // connections is found.
        let cuts = find_cuts(&components, &connections);
        if cuts.len() == 3 {
            for cut in cuts {
                let n1 = &components[cut.component1];
                let n2 = &components[cut.component2];
                println!("Found cut {} -- {}", n1, n2);

                if let Some(index) = connections.iter().position(|c| c == &cut) {
                    connections.remove(index);
                } else {
                    panic!("Did not find cut!");
                }
            }
            break;
        } else {
            println!("Found cut is too large ({})", cuts.len());
        }
    }

    let group_lengths = count_groups(&connections);
    let result = group_lengths.iter().fold(1, |acc, len| acc * len);
    println!("Part 1: {}", result);
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
