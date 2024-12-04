use std::collections::HashSet;

advent_of_code::solution!(4);

struct Grid {
    grid: Vec<Vec<String>>,
    h: usize,
    w: usize,
}

impl Grid {
    pub fn new(input: &str) -> Self {
        let grid: Vec<Vec<String>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_string()).collect())
            .collect();
        let h = grid.len();
        let w = grid[0].len();
        Self { grid, h, w }
    }
    pub fn get_next_n_char(
        &self,
        from_x: usize,
        from_y: usize,
        v_x: i32,
        v_y: i32,
        n: i32,
    ) -> Option<String> {
        let mut word = String::new();
        for i in 0..n {
            let (coord_x, coord_y) = (from_x as i32 + i * v_x, from_y as i32 + i * v_y);
            if let Some(char) = self.get(coord_x as usize, coord_y as usize) {
                word.push_str(&char);
            } else {
                return None;
            }
        }
        Some(word)
    }

    fn get(&self, x: usize, y: usize) -> Option<String> {
        self.grid.get(y).and_then(|row| row.get(x)).cloned()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let mut acc = 0_u32;
    for v_x in -1..2 {
        for v_y in -1..2 {
            if v_x == 0 && v_y == 0 {
                continue;
            }
            for x in 0..grid.w {
                for y in 0..grid.h {
                    if let Some(word) = grid.get_next_n_char(x, y, v_x, v_y, 4) {
                        if word == "XMAS" {
                            acc += 1;
                        }
                    }
                }
            }
        }
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let mut centers = Vec::new();
    for v_x in -1..2 {
        for v_y in -1..2 {
            if v_x == 0 && v_y == 0 {
                continue;
            }
            for x in 0..grid.w {
                for y in 0..grid.h {
                    if let Some(word) = grid.get_next_n_char(x, y, v_x, v_y, 3) {
                        if word == "MAS" {
                            centers.push((x as i32 + v_x, y as i32 + v_y, v_x, v_y));
                        }
                    }
                }
            }
        }
    }
    // Extract all crosses in centers in a dedicated vector
    let mut duplicates = HashSet::new();
    for (x, y, v_x, v_y) in centers.iter() {
        if centers
            .iter()
            .filter(|(x2, y2, v_x2, v_y2)| {
                x == x2 && y == y2 && v_x.abs() == v_x2.abs() && v_y.abs() == v_y2.abs()
            })
            .count()
            > 1
        {
            duplicates.insert((*x, *y));
        }
    }
    Some(duplicates.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
