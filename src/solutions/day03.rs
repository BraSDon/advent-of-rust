use regex::Regex;

struct Coordinate {
    x: u32,
    y: u32,
}

impl Coordinate {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }

    fn adjacent_to(&self, other: &Coordinate) -> bool {
        let x_diff = (self.x as i32 - other.x as i32).abs();
        let y_diff = (self.y as i32 - other.y as i32).abs();

        x_diff <= 1 && y_diff <= 1
    }
}

struct Number {
    value: u32,
    positions: Vec<Coordinate>,
}

impl Number {
    fn new(value: u32, positions: Vec<Coordinate>) -> Self {
        Self { value, positions }
    }

    fn adjacent_to(&self, symbol_position: &Coordinate) -> bool {
        for pos in &self.positions {
            if pos.adjacent_to(symbol_position) {
                return true;
            }
        }
        false
    }
}

fn get_symbol_positions(input: &str) -> Vec<Coordinate> {
    let re = Regex::new(r"[^\d.]").unwrap();
    input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            re.find_iter(line)
                .map(move |m| Coordinate::new(m.start() as u32, y as u32))
        })
        .collect()
}

fn get_numbers(input: &str) -> Vec<Number> {
    let re = Regex::new(r"\d+").unwrap();
    let get_coordinates = |start, length, row| {
        let mut out = vec![];
        for i in start..start + length {
            out.push(Coordinate::new(i, row));
        }
        out
    };

    input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            re.find_iter(line)
                .map(move |m| {
                    let val = m.as_str().parse::<u32>().unwrap();
                    let start = m.start() as u32;
                    let length = m.as_str().len() as u32;
                    Number::new(val, get_coordinates(start, length, y as u32))
                })
        })
        .collect()
}

pub fn part_one(input: &str) -> u32 {
    let symbol_positions = get_symbol_positions(input);
    let numbers = get_numbers(input);

    let mut sum = 0;
    for num in &numbers {
        for pos in &symbol_positions {
            if num.adjacent_to(pos) {
                sum += num.value;
            }
        }
    }
    sum
}

pub fn part_two(input: &str) -> u32 {
    let symbol_positions = get_symbol_positions(input);
    let numbers = get_numbers(input);

    let mut sum = 0;
    for pos in &symbol_positions {
        let mut adj_numbers: Vec<&Number> = Vec::new();
        for num in &numbers {
            if num.adjacent_to(pos) {
                adj_numbers.push(num);
            }
        }

        if adj_numbers.len() == 2 {
            sum += adj_numbers[0].value * adj_numbers[1].value;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 3);
        assert_eq!(part_one(&input), 535078);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("inputs", 3);
        assert_eq!(part_two(&input), 75312571);
    }
}
