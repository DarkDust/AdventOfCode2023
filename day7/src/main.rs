use std::cmp::Ordering;
use std::error::Error;
use std::time::Instant;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: [i32; 5],
    strength: Strength,
    bid: i32,
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let strength_order = self.strength.cmp(&other.strength);
        match strength_order {
            Ordering::Less => return Ordering::Less,
            Ordering::Greater => return Ordering::Greater,
            Ordering::Equal => (),
        }

        for i in 0..5 {
            let card_order = self.cards[i].cmp(&other.cards[i]);
            match card_order {
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
                Ordering::Equal => (),
            }
        }

        Ordering::Equal
    }
}

fn char_to_value1(c: char) -> i32 {
    match c {
        '2'..='9' => (c as i32) - 48,
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid char '{c}'"),
    }
}

fn char_to_value2(c: char) -> i32 {
    match c {
        '2'..='9' => (c as i32) - 48,
        'T' => 10,
        'J' => 0,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => panic!("Invalid char '{c}'"),
    }
}

fn cards_to_strength(cards: [i32; 5]) -> Strength {
    let mut counts: [i32; 5] = [0; 5];
    let mut mapping: [i32; 16] = [-1; 16];
    let mut distinct: i32 = 0;

    // Count how many distinct cards there are.
    for card in cards {
        let index: i32 = mapping[card as usize];
        if index == -1 {
            mapping[card as usize] = distinct;
            counts[distinct as usize] += 1;
            distinct += 1;
        } else {
            counts[index as usize] += 1
        }
    }

    let joker_i = mapping[0];
    if joker_i != -1 && distinct > 1 {
        // Handle Jokers. "Fold" them into the highest count and remove them.
        let mut max_i = 0;
        let mut max_count = 0;
        for k in 0..distinct {
            if k == joker_i {
                continue;
            }

            let count = counts[k as usize];
            if count > max_count {
                max_i = k;
                max_count = count;
            }
        }

        counts[max_i as usize] += counts[joker_i as usize];
        if joker_i < (distinct - 1) {
            counts[joker_i as usize] = counts[(distinct - 1) as usize];
        }
        distinct -= 1;
    }

    match distinct {
        1 => Strength::FiveOfAKind,
        2 => {
            if counts[0] == 1 || counts[1] == 1 {
                Strength::FourOfAKind
            } else {
                Strength::FullHouse
            }
        }
        3 => {
            if counts[0] == 3 || counts[1] == 3 || counts[2] == 3 {
                Strength::ThreeOfAKind
            } else {
                Strength::TwoPair
            }
        }
        4 => Strength::OnePair,
        5 => Strength::HighCard,
        _ => panic!("Cannot be reached"),
    }
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        let mut chars = line.chars();

        let cards: [i32; 5] = [
            char_to_value1(chars.next().unwrap()),
            char_to_value1(chars.next().unwrap()),
            char_to_value1(chars.next().unwrap()),
            char_to_value1(chars.next().unwrap()),
            char_to_value1(chars.next().unwrap()),
        ];

        // Skip space.
        chars.next();

        let strength = cards_to_strength(cards);

        hands.push(Hand {
            cards,
            strength,
            bid: chars.as_str().parse()?,
        });
    }

    hands.sort();

    let mut rank = 1;
    let total = hands.iter().fold(0, |acc, hand| {
        let value = acc + (hand.bid * rank);
        rank += 1;
        return value;
    });

    println!("Part 1: {total}");
    return Ok(());
}

fn part2(input: &str) -> Result<(), Box<dyn Error>> {
    let mut hands: Vec<Hand> = Vec::new();

    for line in input.lines() {
        let mut chars = line.chars();

        let cards: [i32; 5] = [
            char_to_value2(chars.next().unwrap()),
            char_to_value2(chars.next().unwrap()),
            char_to_value2(chars.next().unwrap()),
            char_to_value2(chars.next().unwrap()),
            char_to_value2(chars.next().unwrap()),
        ];

        // Skip space.
        chars.next();

        let strength = cards_to_strength(cards);

        hands.push(Hand {
            cards,
            strength,
            bid: chars.as_str().parse()?,
        });
    }

    hands.sort();

    let mut rank = 1;
    let total = hands.iter().fold(0, |acc, hand| {
        let value = acc + (hand.bid * rank);
        rank += 1;
        return value;
    });

    println!("Part 2: {total}");
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
