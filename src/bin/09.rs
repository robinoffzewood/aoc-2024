advent_of_code::solution!(9);

fn parse_input(input: &str) -> Vec<u64> {
    let line = input.lines().next().unwrap();
    line.chars()
        .map(|line| line.to_digit(10).unwrap() as u64)
        .collect()
}

fn make_bocks(disk_map: &[u64]) -> Vec<Option<u64>> {
    let mut blocks = Vec::new();
    let mut id = 0;
    for (index, map) in disk_map.iter().enumerate() {
        if index % 2 == 0 {
            for _ in 0..*map {
                blocks.push(Some(id));
            }
            id += 1;
        } else {
            for _ in 0..*map {
                blocks.push(None);
            }
        }
    }
    blocks
}

fn move_blocks(blocks: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut moved_blocks = blocks.clone();
    let mut cloned_blocks = moved_blocks.clone();
    for (i, block) in moved_blocks.iter_mut().enumerate() {
        if block.is_none() {
            loop {
                if let Some(Some(last_some)) = cloned_blocks.pop() {
                    *block = Some(last_some);
                    break;
                }
            }
        }
        if i >= cloned_blocks.len() {
            *block = None;
        }
    }
    moved_blocks
}

fn move_blocks_2(blocks: Vec<Option<u64>>) -> Vec<Option<u64>> {
    let mut moved_blocks = blocks.clone();
    let mut moving_blocks = 0;
    let mut skipping_blocks = 0;
    for (i, block) in blocks.iter().rev().enumerate() {
        let index = blocks.len() - i - 1;
        // print_blocks(moved_blocks.clone());
        if block.is_none() {
            continue;
        }
        if moving_blocks > 0 {
            moved_blocks[index] = None;
            moving_blocks -= 1;
            continue;
        }
        if skipping_blocks > 0 {
            skipping_blocks -= 1;
            continue;
        }
        let block_size = get_block_size(&moved_blocks, block.unwrap());
        if let Some(insert_index) = get_next_slot_of_size_n(&moved_blocks, block_size) {
            if insert_index < index {
                moving_blocks = block_size - 1;
                for j in 0..block_size {
                    moved_blocks[insert_index + j] = Some(block.unwrap());
                }
                moved_blocks[index] = None;
                continue;
            }
        }
        skipping_blocks = block_size - 1;
    }
    // print_blocks(moved_blocks.clone());
    moved_blocks
}

fn get_block_size(blocks: &[Option<u64>], id: u64) -> usize {
    blocks
        .iter()
        .filter(|block| block.is_some() && block.unwrap() == id)
        .count()
}

fn get_next_slot_of_size_n(blocks: &[Option<u64>], n: usize) -> Option<usize> {
    let mut contiguous_none = 0;
    for (i, block) in blocks.iter().enumerate() {
        if block.is_some() {
            contiguous_none = 0;
            continue;
        }
        contiguous_none += 1;
        if contiguous_none == n {
            return Some(i - n + 1);
        }
    }
    None
}

fn compute_checksum(blocks: Vec<Option<u64>>) -> u64 {
    let mut checksum = 0;
    for (index, block) in blocks.into_iter().enumerate() {
        if let Some(id) = block {
            checksum += index as u64 * id;
        }
    }
    checksum
}

#[allow(dead_code)]
fn print_blocks(blocks: Vec<Option<u64>>) {
    for block in blocks {
        if let Some(id) = block {
            print!("{}", id);
        } else {
            print!(".");
        }
    }
    println!();
}

pub fn part_one(input: &str) -> Option<u64> {
    let disk_map = parse_input(input);
    let blocks = make_bocks(&disk_map);
    let moved_blocks = move_blocks(blocks);
    let checksum = compute_checksum(moved_blocks);
    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let disk_map = parse_input(input);
    let blocks = make_bocks(&disk_map);
    let moved_blocks = move_blocks_2(blocks);
    let checksum = compute_checksum(moved_blocks);
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
