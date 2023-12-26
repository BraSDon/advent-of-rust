fn hash_string(s: &str) -> u32 {
    let mut curr_val = 0;
    for c in s.chars() {
        let num = c as u32;
        curr_val = ((curr_val + num) * 17) % 256;
    }
    curr_val
}

pub fn part_one(input: &str) -> u32 {
    input
        .split(",")
        .map(hash_string)
        .sum()
}

pub fn part_two(input: &str) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 15);
        assert_eq!(part_one(&input), 511215);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 25);
        assert_eq!(part_two(&input), 0);
    }
}
