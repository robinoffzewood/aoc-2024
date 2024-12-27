use std::collections::HashMap;

advent_of_code::solution!(11);

#[derive(Debug)]
struct Stones {
    stones: Vec<u64>,
    stone_cache: HashMap<(u64, u32), u64>,
}

impl Stones {
    fn parse_input(input: &str) -> Self {
        let stones = input
            .split_whitespace()
            .map(|line| line.parse().unwrap())
            .collect();
        Stones {
            stones,
            stone_cache: HashMap::new(),
        }
    }

    pub fn process(mut self, iterations: u32) -> u64 {
        self.stones
            .clone()
            .into_iter()
            .map(|s| self.blink(s, iterations))
            .sum()
    }

    fn blink(&mut self, stone: u64, depth: u32) -> u64 {
        if depth == 0 {
            return 1;
        }
        let parameters = (stone, depth);
        if self.stone_cache.contains_key(&parameters) {
            return *self.stone_cache.get(&parameters).unwrap();
        }

        let size = if stone == 0 {
            self.blink(1, depth - 1)
        } else if stone.to_string().len() % 2 == 0 {
            let stone_str = stone.to_string();
            let split = stone_str.split_at(stone_str.len() / 2);
            let left = u64::from_str_radix(&split.0, 10).unwrap();
            let right = u64::from_str_radix(&split.1, 10).unwrap();
            let left_size = self.blink(left, depth - 1);
            let right_size = self.blink(right, depth - 1);
            left_size + right_size
        } else {
            self.blink(stone * 2024, depth - 1)
        };
        self.stone_cache.insert(parameters, size);
        size
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let stones = Stones::parse_input(input);
    Some(stones.process(25))
}

pub fn part_two(input: &str) -> Option<u64> {
    let stones = Stones::parse_input(input);
    Some(stones.process(75))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(65601038650482));
    }
}
