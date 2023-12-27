use std::error::Error;
use std::io;

type Src = u32;
type Dest = u32;

#[derive(Clone, Copy)]
struct Range {
    dest_start: Dest,
    src_start: Src,
    length: u32
}

impl Range {
    fn new(dest_start: Dest, src_start: Src, length: u32) -> Self {
        Range {dest_start, src_start, length}
    }

    fn from_line(line: &str) -> Result<Self, io::Error> {
        let nums: Vec<u32> = line.split(" ")
            .filter_map(|s| s.parse().ok())
            .collect();
        if nums.len() == 3 {
            Ok(Range::new(nums[0], nums[1], nums[2]))
        } else {
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "Range line does not contain exactly three numbers")
            )
        }
    }

    fn map(&self, src_val: Src) -> Dest {
        // I assume the caller checked in_range, 
        // therefore offset is always Some and it is safe to unwrap
        self.dest_start + self.offset(src_val).unwrap()
    }

    fn in_range(&self, src_val: Src) -> bool {
        let offset = match self.offset(src_val) {
            Some(offset) => offset,
            None => {return false}
        };
        offset < self.length
    }

    fn offset(&self, src_val: Src) -> Option<u32> {
        src_val.checked_sub(self.src_start)
    }
}

struct Map {
    ranges: Vec<Range>
}

impl Map {
    fn new(ranges: Vec<Range>) -> Self {
        Map {ranges}
    }

    fn map(&self, src_val: Src) -> Dest {
        for range in &self.ranges {
            if range.in_range(src_val) {
                return range.map(src_val);
            }
        }
        return src_val;
    }
}

fn parse_seeds(input: &str) -> Result<Vec<u32>, Box<dyn Error>> {
    let line = input.lines().nth(0).ok_or("")?;
    Ok(line.split(" ")
        .into_iter()
        .filter_map(|s| s.parse().ok())
        .collect::<Vec<u32>>())
}

fn parse_maps(input: &str) -> Vec<Map> {
    let mut ranges = vec!();
    let mut curr = vec!(); 
    let mut skip_next = false;
    for line in input.lines().skip(3) {
        if skip_next {
            skip_next = false;
            continue;
        }
        if line.trim().is_empty() {
            ranges.push(Map::new(curr));
            curr = vec!();
            skip_next = true;
            continue;
        }
        match Range::from_line(line) {
            Ok(range) => curr.push(range),
            Err(e) => panic!("{}", e)
        }
    }
    // finally add the last map
    ranges.push(Map::new(curr));
    ranges
}

pub fn part_one(input: &str) -> u32 {
    let seeds = match parse_seeds(input) {
        Ok(seeds) => seeds,
        Err(e) => panic!("{}", e)
    };
    let maps = parse_maps(input);
    let mut locations = vec!();
    for seed in seeds {
        let mut curr = seed;
        for map in &maps {
            curr = map.map(curr);
        }
        locations.push(curr);
    }

    match locations.iter().min() {
        Some(min) => *min,
        _ => panic!("locations are empty")
    }
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
        let input = read_file("inputs", 5);
        assert_eq!(part_one(&input), 621354867);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 25);
        assert_eq!(part_two(&input), 0);
    }
}
