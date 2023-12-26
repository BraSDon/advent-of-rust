use std::error::Error;
use regex::Regex;

struct Race {
    time: u64,
    record: u64
}

impl Race {
    fn new(time: u64, record: u64) -> Self {
        Self { time, record }
    }

    fn winning(&self, charging: u64) -> bool {
        (charging * (self.time - charging)) > self.record
    }

    fn calculate_wins(&self) -> u32 {
        let mut fst = 1;
        let mut snd = self.time - 1;
        loop {
            let fst_winning = self.winning(fst);
            let snd_winning = self.winning(snd);

            if fst_winning && snd_winning {
                return (snd - fst + 1) as u32;
            }
            if !fst_winning {
                fst += 1;
            }
            if !snd_winning {
                snd -= 1;
            }
        }
    }
}

fn parse_part_one(input: &str) -> Result<Vec<Race>, Box<dyn Error>> {
    let first = input.lines().nth(0).unwrap();
    let second = input.lines().nth(1).unwrap();

    let re = Regex::new(r"(\d+)")?;
    let first_numbers: Vec<u64> = re.find_iter(first)
        .map(|m| m.as_str().parse().unwrap())
        .collect();
    let second_numbers: Vec<u64> = re.find_iter(second)
        .map(|m| m.as_str().parse().unwrap())
        .collect();

    let races: Vec<Race> = first_numbers.into_iter()
        .zip(second_numbers.into_iter())
        .map(|(time, record)| Race::new(time, record))
        .collect();

    Ok(races)
}

fn parse_part_two(input: &str) -> Result<Race, Box<dyn Error>> {
    let re = Regex::new(r"Time:\s*((\d+\s*)+)")?;
    let first_line = input.lines().nth(0).ok_or("No first line")?;
    let cap = re.captures(first_line).ok_or("No captures")?;
    let time_string = cap[1].replace(" ", "");
    
    let re = Regex::new(r"Distance:\s*((\d+\s*)+)")?;
    let second_line = input.lines().nth(1).ok_or("No second line")?;
    let cap = re.captures(second_line).ok_or("No captures")?;
    let record_string = cap[1].replace(" ", "");

    let time = time_string.parse::<u64>()?;
    let record = record_string.parse::<u64>()?;

    Ok(Race::new(time, record))
}

pub fn part_one(input: &str) -> u32 {
    let races = match parse_part_one(input) {
        Ok(races) => races,
        Err(e) => panic!("Error parsing input: {}", e)
    };

    races.iter()
        .map(Race::calculate_wins)
        .product()
}

pub fn part_two(input: &str) -> u32 {
    match parse_part_two(input) {
        Ok(race) => race.calculate_wins(),
        Err(e) => panic!("Error parsing input: {}", e)
    }
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 6);
        assert_eq!(part_one(&input), 449550);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("inputs", 6);
        assert_eq!(part_two(&input), 28360140);
    }
}
