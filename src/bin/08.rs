use std::collections::{HashMap, HashSet};

advent_of_code::solution!(8);

type Grid = HashSet<(usize, usize)>;

#[derive(Debug)]
struct Antenna {
    grid: Grid,
    w: usize,
    h: usize,
}

fn parse(input: &str) -> HashMap<char, Antenna> {
    let mut antennas = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let antenna = antennas.entry(char).or_insert(Antenna {
                grid: Grid::default(),
                w: line.len(),
                h: input.lines().count(),
            });
            antenna.grid.insert((x, y));
        }
    }
    antennas
}

fn is_in_bounds(antenna: &Antenna, loc: (isize, isize)) -> bool {
    loc.0 >= 0 && loc.0 < antenna.w as isize && loc.1 >= 0 && loc.1 < antenna.h as isize
}

fn compute_antinode_part_1(antenna: &Antenna) -> Grid {
    let mut antinode = Grid::default();
    for (i, loc_1) in antenna.grid.iter().enumerate() {
        for j in i + 1..antenna.grid.len() {
            let loc_2 = antenna.grid.iter().nth(j).unwrap();
            let (step_x, step_y) = (
                (loc_2.0 as isize - loc_1.0 as isize),
                (loc_2.1 as isize - loc_1.1 as isize),
            );
            let backward = (loc_1.0 as isize - step_x, loc_1.1 as isize - step_y);
            if is_in_bounds(antenna, backward) {
                antinode.insert((backward.0 as usize, backward.1 as usize));
            }
            let forward = (loc_2.0 as isize + step_x, loc_2.1 as isize + step_y);
            if is_in_bounds(antenna, forward) {
                antinode.insert((forward.0 as usize, forward.1 as usize));
            }
        }
    }
    antinode
}
fn compute_antinode_part_2(antenna: &Antenna) -> Grid {
    let mut antinode = Grid::default();
    for (i, loc_1) in antenna.grid.iter().enumerate() {
        for j in i + 1..antenna.grid.len() {
            let loc_2 = antenna.grid.iter().nth(j).unwrap();
            let (step_x, step_y) = (
                (loc_2.0 as isize - loc_1.0 as isize),
                (loc_2.1 as isize - loc_1.1 as isize),
            );
            let mut counter = 0;
            loop {
                counter += 1;
                let backward = (
                    loc_2.0 as isize - counter * step_x,
                    loc_2.1 as isize - counter * step_y,
                );
                if is_in_bounds(antenna, backward) {
                    antinode.insert((backward.0 as usize, backward.1 as usize));
                } else {
                    break;
                }
            }
            counter = 0;
            loop {
                counter += 1;
                let forward = (
                    loc_1.0 as isize + counter * step_x,
                    loc_1.1 as isize + counter * step_y,
                );
                if is_in_bounds(antenna, forward) {
                    antinode.insert((forward.0 as usize, forward.1 as usize));
                } else {
                    break;
                }
            }
        }
    }
    antinode
}

pub fn part_one(input: &str) -> Option<u32> {
    let antennas = parse(input);
    let mut all_antinodes = Grid::default();
    for antenna in antennas.values() {
        let antinode = compute_antinode_part_1(antenna);
        all_antinodes.extend(antinode);
    }
    Some(all_antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let antennas = parse(input);
    let mut all_antinodes = Grid::default();
    for antenna in antennas.values() {
        let antinode = compute_antinode_part_2(antenna);
        all_antinodes.extend(antinode);
    }
    Some(all_antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
