advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = parse(input);
    left_list.sort();
    right_list.sort();
    let mut acc: i32 = 0;
    for (index, item) in left_list.iter().enumerate() {
        acc += (item - right_list.get(index).unwrap()).abs();
    }
    Some(acc as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = parse(input);
    let mut acc: i32 = 0;
    for left in left_list {
        for right in right_list.iter() {
            if right == &left {
                acc += left;
            }
        }
    }
    Some(acc as u32)
}

pub fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list = Vec::new();
    let mut right_list = Vec::new();
    for line in input.lines() {
        let split_str: Vec<&str> = line.split_whitespace().collect();
        let l_u32 = split_str[0].parse::<i32>().unwrap();
        let r_u32 = split_str[1].parse::<i32>().unwrap();
        left_list.push(l_u32);
        right_list.push(r_u32);
    }
    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
