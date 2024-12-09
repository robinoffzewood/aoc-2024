use std::collections::HashMap;

advent_of_code::solution!(6);

#[derive(Clone, PartialEq)]
enum Cell {
    Empty,
    Obstacle,
}

#[derive(Clone, PartialEq)]
enum EndState {
    OutOfGrid,
    Normal,
}

/// 0,0 is top left
#[derive(Clone)]
struct Grid {
    grid: Vec<Vec<Cell>>,
    guard_x: i32,
    guard_y: i32,
    guard_vx: i32,
    guard_vy: i32,
    distinct_positions: HashMap<(i32, i32), (i32, i32)>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let mut grid = Vec::new();
        let mut guard_x = 0;
        let mut guard_y = 0;
        let guard_vx = 0;
        let guard_vy = -1;
        let mut distinct_positions = HashMap::new();
        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => row.push(Cell::Empty),
                    '#' => row.push(Cell::Obstacle),
                    '^' => {
                        row.push(Cell::Empty);
                        guard_x = x as i32;
                        guard_y = y as i32;
                        distinct_positions.insert((guard_x, guard_y), (0, 0));
                    }
                    _ => panic!("Invalid character in input"),
                }
            }
            grid.push(row);
        }
        Self {
            guard_x,
            guard_y,
            guard_vx,
            guard_vy,
            grid,
            distinct_positions,
        }
    }

    fn move_guard(&mut self) -> EndState {
        // Depending on where the guard would be after next step, either move it, turn it, or return False meaning it left the grid
        let next_x = self.guard_x + self.guard_vx;
        let next_y = self.guard_y + self.guard_vy;
        if next_y < 0
            || next_y >= self.grid.len() as i32
            || next_x < 0
            || next_x >= self.grid[0].len() as i32
        {
            return EndState::OutOfGrid;
        }
        match self.grid[next_y as usize][next_x as usize] {
            Cell::Empty => {
                self.guard_x = next_x;
                self.guard_y = next_y;
            }
            Cell::Obstacle => {
                // turn right
                if self.guard_vx == 0 && self.guard_vy == -1 {
                    self.guard_vx = 1;
                    self.guard_vy = 0;
                } else if self.guard_vx == 0 && self.guard_vy == 1 {
                    self.guard_vx = -1;
                    self.guard_vy = 0;
                } else if self.guard_vx == -1 && self.guard_vy == 0 {
                    self.guard_vx = 0;
                    self.guard_vy = -1;
                } else if self.guard_vx == 1 && self.guard_vy == 0 {
                    self.guard_vx = 0;
                    self.guard_vy = 1;
                }
                self.guard_x += self.guard_vx;
                self.guard_y += self.guard_vy;
            }
        }
        EndState::Normal
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::new(input);
    while grid.move_guard() != EndState::OutOfGrid {
        grid.distinct_positions
            .insert((grid.guard_x, grid.guard_y), (0, 0));
    }
    Some(grid.distinct_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = Grid::new(input);
    let mut obstacles = Vec::new();
    // Try to place an obstacle in all the Empty positions of the grid, and move the guard until it leaves the grid or is stuck in a loop
    // To know that the guard is stuck in a loop, when the  guard is in a location where she's already been,
    // we compare the number of distinct positions before and now. If they are the same, the guard is stuck in a loop.
    for obstacle_x in 0..grid.grid[0].len() {
        for obstacle_y in 0..grid.grid.len() {
            // Skip the initial guard position
            if obstacle_x == grid.guard_x as usize && obstacle_y == grid.guard_y as usize {
                continue;
            }
            // Skip if there is already an obstacle
            if grid.grid[obstacle_y][obstacle_x] == Cell::Obstacle {
                continue;
            }
            // modify the grid
            let mut modified_grid = grid.clone();
            modified_grid.grid[obstacle_y][obstacle_x] = Cell::Obstacle;
            let mut stuck_in_loop = false;
            while modified_grid.move_guard() != EndState::OutOfGrid {
                let (mut current_vx, mut current_vy) =
                    (modified_grid.guard_vx, modified_grid.guard_vy);
                if let Some((previous_vx, previous_vy)) = modified_grid
                    .distinct_positions
                    .get_mut(&(modified_grid.guard_x, modified_grid.guard_y))
                {
                    if (&previous_vx, &previous_vy) == (&&mut current_vx, &&mut current_vy) {
                        stuck_in_loop = true;
                        break;
                    }
                    (*previous_vx, *previous_vy) = (current_vx, current_vy);
                } else {
                    modified_grid.distinct_positions.insert(
                        (modified_grid.guard_x, modified_grid.guard_y),
                        (current_vx, current_vy),
                    );
                }
            }
            if stuck_in_loop {
                obstacles.push((obstacle_x, obstacle_y));
            }
        }
    }
    Some(obstacles.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
