use regex::Regex;
use std::cmp;
use std::error::Error;
use std::time::Instant;

#[derive(Debug)]
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let line_re = Regex::new(r"^Game (?<id>\d+): (?<turns>.*)$")?;
    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let captures = line_re.captures(line).unwrap();
        let mut game = Game {
            id: captures["id"].parse()?,
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };

        for turn in captures["turns"].split(";") {
            for info in turn.split(",").map(|s| s.trim()) {
                let parts: Vec<&str> = info.split(" ").collect();
                let num: u32 = parts[0].parse()?;
                match parts[1] {
                    "red" => game.max_red = cmp::max(game.max_red, num),
                    "green" => game.max_green = cmp::max(game.max_green, num),
                    "blue" => game.max_blue = cmp::max(game.max_blue, num),
                    _ => panic!(),
                }
            }
        }

        games.push(game);
    }

    let sum: u32 = games
        .iter()
        .filter(|g| {
            return g.max_red <= 12 && g.max_green <= 13 && g.max_blue <= 14;
        })
        .fold(0, |acc, g| acc + g.id);

    println!("Part 1: {}", sum);
    return Ok(());
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let line_re = Regex::new(r"^Game (?<id>\d+): (?<turns>.*)$")?;
    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        let captures = line_re.captures(line).unwrap();
        let mut game = Game {
            id: captures["id"].parse()?,
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        };

        for turn in captures["turns"].split(";") {
            for info in turn.split(",").map(|s| s.trim()) {
                let parts: Vec<&str> = info.split(" ").collect();
                let num: u32 = parts[0].parse()?;
                match parts[1] {
                    "red" => game.max_red = cmp::max(game.max_red, num),
                    "green" => game.max_green = cmp::max(game.max_green, num),
                    "blue" => game.max_blue = cmp::max(game.max_blue, num),
                    _ => panic!(),
                }
            }
        }

        games.push(game);
    }

    let sum: u32 = games.iter().fold(0, |acc, game| {
        acc + (game.max_red * game.max_green * game.max_blue)
    });

    println!("Part 2: {}", sum);
    return Ok(());
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../rsc/input.txt");

    let start1 = Instant::now();
    part1(input)?;
    println!("Elapsed: {:.2?}\n", start1.elapsed());

    let start2 = Instant::now();
    part2(input)?;
    println!("Elapsed: {:.2?}", start2.elapsed());

    Ok(())
}
