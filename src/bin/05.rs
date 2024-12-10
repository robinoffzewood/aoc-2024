use std::collections::HashMap;

advent_of_code::solution!(5);

fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();
    let mut first_section = true;
    for line in input.lines() {
        if line.is_empty() {
            first_section = false;
            continue;
        }
        if first_section {
            let (left, right) = line.split_once("|").unwrap();
            let (left, right) = (left.parse::<u32>().unwrap(), right.parse::<u32>().unwrap());
            rules
                .entry(left)
                .and_modify(|v: &mut Vec<u32>| v.push(right))
                .or_insert(vec![right]);
        } else {
            let mut update = Vec::new();
            line.split(',').for_each(|n| {
                update.push(n.parse::<u32>().unwrap());
            });
            updates.push(update);
        }
    }
    (rules, updates)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    let mut acc = 0;
    'loop_update: for update in updates {
        let mut previous_nb = Vec::new();
        for nb in &update {
            if let Some(rule) = rules.get(nb) {
                for r in rule {
                    if previous_nb.contains(r) {
                        continue 'loop_update;
                    }
                }
            }
            previous_nb.push(*nb);
        }
        acc += update.get((update.len() - 1) >> 1).unwrap();
    }
    Some(acc)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, updates) = parse(input);
    let mut acc = 0;
    let mut incorrect_updates = Vec::new();
    'loop_update: for update in updates {
        let mut previous_nb = Vec::new();
        for nb in &update {
            if let Some(rule) = rules.get(nb) {
                for r in rule {
                    if previous_nb.contains(r) {
                        incorrect_updates.push(update);
                        continue 'loop_update;
                    }
                }
            }
            previous_nb.push(*nb);
        }
    }
    let mut corrected_updates = Vec::new();
    for update in incorrect_updates {
        let mut corrected_update = Vec::new();
        let mut valid_rules = HashMap::new();
        for nb in &update {
            valid_rules.insert(*nb, 0);
            if let Some(rule) = rules.get(nb) {
                for r in rule {
                    if update.contains(r) {
                        valid_rules.entry(*nb).and_modify(|v| *v += 1);
                    }
                }
            }
        }
        while !valid_rules.is_empty() {
            valid_rules.retain(|k, v| {
                if *v == 0 {
                    corrected_update.push(*k);
                    false
                } else {
                    *v -= 1;
                    true
                }
            });
        }
        corrected_updates.push(corrected_update);
    }
    for update in corrected_updates {
        acc += update.get((update.len() - 1) >> 1).unwrap();
    }
    Some(acc)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
