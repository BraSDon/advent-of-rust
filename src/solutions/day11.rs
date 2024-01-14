use core::panic;

use grid::Grid;

struct Coordinate {
    x: usize,
    y: usize
}

impl Coordinate {
    fn manhatten_distance(&self, other: &Coordinate) -> usize {
        let x_dist = (self.x as i32 - other.x as i32).abs();
        let y_dist = (self.y as i32 - other.y as i32).abs();
        (x_dist + y_dist) as usize
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
enum Symbol {
    #[default] Dot,
    Hashtag
}

impl Symbol {
    fn from_char(c: char) -> Self {
        match c {
            '.' => Symbol::Dot,
            '#' => Symbol::Hashtag,
            _ => panic!("invalid symbol")
        }
    }

    fn is_dot(symbol: &Symbol) -> bool {
        *symbol == Symbol::Dot
    }
}

struct Universe {
    grid: Grid<Symbol>,
    galaxies: Vec<Coordinate>
}

impl Universe {
    fn new(grid: Grid<Symbol>) -> Self {
        Universe { grid, galaxies: vec![] }
    }

    fn expand(&mut self) {
        let mut empty_row_indices = vec![];
        for (i, mut row) in self.grid.iter_rows().enumerate() {
            if row.all(Symbol::is_dot) {
                empty_row_indices.push(i);
            }
        }

        let mut empty_col_indices = vec![];
        for (i, mut col) in self.grid.iter_cols().enumerate() {
            if col.all(Symbol::is_dot) {
                empty_col_indices.push(i);
            }
        }

        for i in empty_row_indices.iter().rev() {
            self.insert_empty(*i, "row")
        }

        for i in empty_col_indices.iter().rev() {
            self.insert_empty(*i, "col")
        }
    }

    fn insert_empty(&mut self, index: usize, rc: &str) {
        match rc {
            "row" => self.grid.insert_row(index, vec![Symbol::Dot; self.grid.cols()]),
            "col" => self.grid.insert_col(index, vec![Symbol::Dot; self.grid.rows()]),
            _ => panic!("custom: invalid row/col identifier")
        }
        
    }

    fn find_galaxies(&mut self) {
        for (i, row) in self.grid.iter_rows().enumerate() {
            for (j, &symbol) in row.enumerate() {
                if symbol != Symbol::Hashtag {
                    continue;
                }
                self.galaxies.push(Coordinate {x: i, y: j});
            }
        }
    }

    fn sum_of_distances(&self) -> u32 {
        let mut sum = 0;
        for i in 0..self.galaxies.len() {
            for j in i+1..self.galaxies.len() {
                let galaxy1 = &self.galaxies[i];
                let galaxy2 = &self.galaxies[j];
                sum += galaxy1.manhatten_distance(&galaxy2) as u32;
            }
        }
        sum
    }
}

fn parse_input(input: &str) -> Grid<Symbol> {
    let mut grid = Grid::new(0, 0);
    for (i, line) in input.lines().enumerate() {
        let row = line.chars().map(Symbol::from_char).collect::<Vec<Symbol>>();
        grid.insert_row(i, row)
    }
    grid
}

pub fn part_one(input: &str) -> u32 {
    let grid = parse_input(input);
    let mut universe = Universe::new(grid);
    universe.expand();
    universe.find_galaxies();
    universe.sum_of_distances()
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
        let input = read_file("inputs", 11);
        assert_eq!(part_one(&input), 9543156);

    }

    #[test]
    fn test_part_two() {
        let input = read_file("examples", 25);
        assert_eq!(part_two(&input), 0);
    }

    #[test]
    fn test_manhatten_distance() {
        let coord1 = Coordinate { x: 1, y: 1 };
        let coord2 = Coordinate { x: 4, y: 5 };
        assert_eq!(coord1.manhatten_distance(&coord2), 7);
    }

    #[test]
    fn test_symbol_from_char() {
        assert_eq!(Symbol::from_char('.'), Symbol::Dot);
        assert_eq!(Symbol::from_char('#'), Symbol::Hashtag);
    }

    #[test]
    #[should_panic(expected = "invalid symbol")]
    fn test_symbol_from_invalid_char() {
        Symbol::from_char('x');
    }

    #[test]
    fn test_symbol_is_dot() {
        assert_eq!(Symbol::is_dot(&Symbol::Dot), true);
        assert_eq!(Symbol::is_dot(&Symbol::Hashtag), false);
    }

    #[test]
    fn test_universe_new() {
        let grid = Grid::new(0, 0);
        let universe = Universe::new(grid);
        assert_eq!(universe.galaxies.len(), 0);
    }
}
