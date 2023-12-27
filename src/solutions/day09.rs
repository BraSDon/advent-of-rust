use std::iter::FromIterator;
use std::ops::Index;

struct Seq {
    seq: Vec<i32>
}

impl FromIterator<i32> for Seq {
    fn from_iter<I: IntoIterator<Item=i32>>(iter: I) -> Self {
        let seq = iter.into_iter().collect();
        Seq { seq }
    }
}

impl Index<usize> for Seq {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.seq[index]
    }
}

impl Seq {
    fn from_line(line: &str) -> Self {
        let seq = line.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        Seq { seq }
    }

    fn diff(&self) -> Seq {
        self.seq
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect()
    }

    fn is_zero(&self) -> bool {
        self.seq.iter().all(|&x| x == 0)
    }

    fn last(&self) -> Option<&i32> {
        self.seq.last()
    }

}

struct History {
    sequences: Vec<Seq>,
    complete: bool
}

impl History {
    fn new(seq: Seq) -> Self {
        History { sequences: vec![seq], complete: false}
    }

    fn step(&mut self) {
        let new_seq = self.sequences.last().unwrap().diff();
        self.complete = new_seq.is_zero();
        self.sequences.push(new_seq);
    }

    fn predict_right(&self) -> i32 {
        let mut curr = 0;
        for seq in self.sequences.iter().rev().skip(1) {
            curr = seq.last().unwrap() + curr;
        }
        curr
    }

    fn predict_left(&self) -> i32 {
        let mut curr = 0;
        for seq in self.sequences.iter().rev().skip(1) {
            curr = seq[0] - curr;
        }
        curr
    }
}

fn solve(input: &str, predict_fn: fn(&History) -> i32) -> u32 {
    let mut histories = input.lines()
        .map(Seq::from_line)
        .map(History::new)
        .collect::<Vec<History>>();

    while histories.iter().any(|history| !history.complete) {
        for history in histories.iter_mut() {
            if !history.complete {
                history.step();
            }
        }
    }

    histories.iter().map(predict_fn).sum::<i32>().try_into().unwrap()
}

pub fn part_one(input: &str) -> u32 {
    solve(input, History::predict_right)
}

pub fn part_two(input: &str) -> u32 {
    solve(input, History::predict_left)
}

#[cfg(test)]
mod tests {
    use crate::read_file;

    use super::*;

    #[test]
    fn test_part_one() {
        let input = read_file("inputs", 9);
        assert_eq!(part_one(&input), 1702218515);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("inputs", 9);
        assert_eq!(part_two(&input), 925);
    }
}
