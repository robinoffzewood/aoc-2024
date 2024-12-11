advent_of_code::solution!(2);

fn is_safe(line: &[i32]) -> u32 {
    let mut old_diff: Option<i32> = None;
    for (index, item) in line.iter().enumerate() {
        if index == line.len() - 1 {
            break;
        }
        let current_diff = line.get(index + 1).unwrap() - item;
        if current_diff.abs() == 0 || current_diff.abs() > 3 {
            return 0;
        }
        if let Some(diff) = old_diff {
            if current_diff.signum() != diff.signum() {
                return 0;
            }
        }
        old_diff = Some(current_diff);
    }
    1
}

pub fn part_one(input: &str) -> Option<u32> {
    let list = parse(input);
    let mut safe_lines = 0;
    for line in list {
        safe_lines += is_safe(&line);
    }
    Some(safe_lines)
}

pub fn part_two(input: &str) -> Option<u32> {
    let list = parse(input);
    let mut safe_lines = 0;
    for line in list {
        if is_safe(&line) == 1 {
            safe_lines += 1;
            continue;
        }
        // Try to remove the levels one after the other, and see if it makes the report safe.
        for index_to_remove in 0..line.len() {
            let mut altered_line = line.clone();
            altered_line.remove(index_to_remove);
            if is_safe(&altered_line) == 1 {
                safe_lines += 1;
                break;
            }
        }
    }
    Some(safe_lines)
}

pub fn parse(input: &str) -> Vec<Vec<i32>> {
    let mut list = Vec::new();
    for line in input.lines() {
        let split_str: Vec<&str> = line.split_whitespace().collect();
        let mut inner_list = Vec::new();
        for item in split_str {
            inner_list.push(item.parse::<i32>().unwrap());
        }
        list.push(inner_list);
    }
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
