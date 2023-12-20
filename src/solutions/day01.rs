fn find_digits(line: &str) -> Vec<u32> {
    return line
        .chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
}

fn combine_first_and_last(digits: Vec<u32>) -> u32 {
    let first = digits[0].to_string();
    let last = digits[digits.len() - 1].to_string();
    format!("{}{}", first, last).parse::<u32>().unwrap()
}

fn replace(line: &str) -> String {
    let replacements = [
        ("one", "o1ne"),
        ("two", "t2wo"),
        ("three", "t3hree"),
        ("four", "f4our"),
        ("five", "f5ive"),
        ("six", "s6ix"),
        ("seven", "s7even"),
        ("eight", "e8ight"),
        ("nine", "n9ne"),
    ];

    let mut result = line.to_string();

    for &(original, replacement) in &replacements {
        result = result.replace(original, replacement);
    }

    result
}

pub fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(find_digits)
        .map(combine_first_and_last)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(replace)
        .map(|line| find_digits(&line))
        .map(combine_first_and_last)
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 1);
        assert_eq!(part_one(&input), 54630);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("inputs", 1);
        assert_eq!(part_two(&input), 54770);
    }
}
