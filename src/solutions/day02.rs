use regex::Regex;

const GREEN_REGEX: &str = r"(\d+) green";
const RED_REGEX: &str = r"(\d+) red";
const BLUE_REGEX: &str = r"(\d+) blue";

fn get(line: &str, re: &Regex) -> Vec<u32> {
    re.captures_iter(line)
        .filter_map(|cap| cap[1].parse().ok())
        .collect()
}

fn extract_game_number(line: &str) -> Result<u32, Box<dyn std::error::Error>> {
    let re = Regex::new(r"Game (\d+):")?;
    let caps = re.captures(line).ok_or("No captures")?;
    let game_number_str = caps.get(1).ok_or("No game number")?.as_str();
    let game_number = game_number_str.parse::<u32>()?;
    Ok(game_number)
}

fn process_line(line: &str) -> (Option<u32>, Option<u32>, Option<u32>) {
    let green_regex = Regex::new(GREEN_REGEX).unwrap();
    let red_regex = Regex::new(RED_REGEX).unwrap();
    let blue_regex = Regex::new(BLUE_REGEX).unwrap();

    let green = get(line, &green_regex);
    let red = get(line, &red_regex);
    let blue = get(line, &blue_regex);

    let max_green = green.iter().max().cloned();
    let max_red = red.iter().max().cloned();
    let max_blue = blue.iter().max().cloned();

    (max_green, max_red, max_blue)
}

pub fn part_one(input: &str) -> u32 {
    input.lines()
        .filter(|line| {
            let (max_green, max_red, max_blue) = process_line(line);
            match (max_green, max_red, max_blue) {
                (Some(green), Some(red), Some(blue)) => {
                    red <= 12 && green <= 13 && blue <= 14
                },
                _ => panic!("max value could not be computed for one of the colors")
            }
        })
        .map(|line| {
            match extract_game_number(line) {
                Ok(game_number) => game_number,
                Err(e) => panic!("Could not extract game number: {}", e)
            }
        })
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input.lines()
        .map(|line| {
            let (max_green, max_red, max_blue) = process_line(line);
            match (max_green, max_red, max_blue) {
                (Some(green), Some(red), Some(blue)) => {
                    red * green * blue
                },
                _ => panic!("max value could not be computed for one of the colors")
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 2);
        assert_eq!(part_one(&input), 2449);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("inputs", 2);
        assert_eq!(part_two(&input), 63981);
    }
}
