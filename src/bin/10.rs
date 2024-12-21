use std::{collections::HashSet, vec};

advent_of_code::solution!(10);

struct Map {
    map: Vec<Vec<u32>>,
    w: usize,
    h: usize,
}

impl Map {
    pub fn new(input: &str) -> Self {
        let map: Vec<Vec<u32>> = input
            .lines()
            .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
            .collect();
        let h = map.len();
        let w = map[0].len();
        Self { map, h, w }
    }

    pub fn trailheads(&self) -> Vec<(usize, usize)> {
        let mut trailheads = Vec::new();
        for y in 0..self.h {
            for x in 0..self.w {
                if self.map[y][x] == 0 {
                    trailheads.push((x, y));
                }
            }
        }
        trailheads
    }

    fn climb_from(&self, x_y: (usize, usize)) -> u32 {
        let mut summits = 0;
        let x_0 = x_y.0;
        let y_0 = x_y.1;
        let mut to_visit = vec![(x_0, y_0)];
        let mut visited = HashSet::new();
        visited.insert((x_0, y_0));

        while let Some((visiting_x, visiting_y)) = to_visit.pop() {
            
            visited.insert((visiting_x, visiting_y));
            let current_height = self.map[visiting_y][visiting_x];
            if current_height == 9 {
                summits += 1;
                continue;
            }
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (x, y) = (visiting_x as i32 + dx, visiting_y as i32 + dy);
                let (x, y) = (x as usize, y as usize);
                if let Some(height) = self.map.get(y).and_then(|row| row.get(x)) {
                    if *height == current_height + 1 && !visited.contains(&(x, y)) {
                        to_visit.push((x, y));
                    }
                }
            }
        }
        summits
    }

    fn climb_from_distinct(&self, x_y: (usize, usize)) -> u32 {
        let mut summits = 0;
        let x_0 = x_y.0;
        let y_0 = x_y.1;
        let mut to_visit = vec![(x_0, y_0)];

        while let Some((visiting_x, visiting_y)) = to_visit.pop() {
            
            let current_height = self.map[visiting_y][visiting_x];
            if current_height == 9 {
                summits += 1;
                continue;
            }
            for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let (x, y) = (visiting_x as i32 + dx, visiting_y as i32 + dy);
                let (x, y) = (x as usize, y as usize);
                if let Some(height) = self.map.get(y).and_then(|row| row.get(x)) {
                    if *height == current_height + 1 {
                        to_visit.push((x, y));
                    }
                }
            }
        }
        summits
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let mut acc = 0;
    for trailhead in map.trailheads() {
        let score = map.climb_from(trailhead);
        acc += score;
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::new(input);
    let mut acc = 0;
    for trailhead in map.trailheads() {
        let score = map.climb_from_distinct(trailhead);
        acc += score;
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(81));
    }
}
