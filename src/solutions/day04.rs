use std::{collections::{HashSet, HashMap}, error::Error};
use itertools::Itertools;
use regex::Regex;


#[derive(Clone, Copy, Debug)]
struct Card {
    id: u32,
    intersection_size: u32,
    frequency: u32,
}

impl Card {
    fn get_points(card: Card) -> u32 {
        if card.intersection_size == 0 {
            return 0;
        }
        u32::pow(2, card.intersection_size - 1)
    }

    fn increase_frequency(&mut self, num: u32) {
        self.frequency += num;
    }
}

fn get_set(s: &str) -> HashSet<i32> {
    let mut hs = HashSet::<i32>::new();
    let re = Regex::new(r"\d+").expect("invalid regex");
    for cap in re.captures_iter(s) {
        hs.insert(cap[0].parse::<i32>().unwrap());
    }
    hs
}

fn parse_line(line: &str) -> Result<Card, Box<dyn Error>> {
    let re = Regex::new(r"Card\s*(\d+): *(\d+.*) *\| *(\d+.*)")?;
    let caps = re.captures(line).ok_or("No captures")?;

    let id = caps[1].parse::<u32>()?;
    let winning_set = get_set(&caps[2]);
    let my_set = get_set(&caps[3]);
    let intersection_size = winning_set.intersection(&my_set).count();

    Ok(Card { id, intersection_size: intersection_size as u32, frequency: 1})
}

pub fn part_one(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            match parse_line(line) {
                Ok(card) => Card::get_points(card),
                Err(e) => panic!("{}", e),
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    let mut cards = input.lines()
        .map(|line| {
            let c = match parse_line(line) {
                Ok(card) => card,
                Err(e) => panic!("{}", e)
            };
            (c.id, c)
        })
        .collect::<HashMap<u32, Card>>();
    // Lessons learned debugging: HashMaps are not sorted
    let ids: Vec<_> = cards.keys().cloned().sorted().collect();
    for id in ids {
        let card = &cards[&id];
        let freq = card.frequency;
        let i_size = card.intersection_size;
        for i in 0..i_size {
            let target = id + i + 1;
            if let Some(target_card) = cards.get_mut(&target) {
                target_card.increase_frequency(freq);
            }
        }
    }

    cards.values().map(|card| card.frequency).sum()
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 4);
        assert_eq!(part_one(&input), 24733);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("inputs", 4);
        assert_eq!(part_two(&input), 5422730);
    }
}
