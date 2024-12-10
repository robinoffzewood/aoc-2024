advent_of_code::solution!(3);

fn get_mul(input: &str) -> u32 {
    let re = regex::Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut acc = 0;
    for (_, [a, b]) in re.captures_iter(input).map(|c| c.extract()) {
        let a = a.parse::<u32>().unwrap();
        let b = b.parse::<u32>().unwrap();
        acc += a * b;
    }
    acc
}

pub fn part_one(input: &str) -> Option<u32> {
    let acc = get_mul(input);
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input = "do".to_owned() + input;
    let mut acc = 0;
    for line_do in input.split("don't") {
        if let Some(do_location) = line_do.find("do") {
            acc += get_mul(line_do.split_at(do_location).1);
        }
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}
