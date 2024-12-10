advent_of_code::solution!(7);

#[derive(Debug, Clone)]
struct Equation {
    test_value: u64,
    numbers: Vec<u64>,
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

fn parse(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(": ");
            let test_value = parts.next().unwrap().parse().unwrap();
            let numbers = parts
                .next()
                .unwrap()
                .split(' ')
                .map(|f| f.parse().unwrap())
                .collect::<Vec<u64>>();
            Equation {
                test_value,
                numbers,
            }
        })
        .collect()
}

fn make_operators_part_1(n: u32) -> Vec<Vec<Operator>> {
    let mut operators_list = Vec::new();
    for i in 0..2_i32.pow(n) {
        let mut operators = Vec::new();
        for j in 0..n {
            if i & (1 << j) == 0 {
                operators.push(Operator::Add);
            } else {
                operators.push(Operator::Multiply);
            }
        }
        operators_list.push(operators);
    }
    operators_list
}

fn get_part_1_result(equations: &Vec<Equation>) -> (u64, Vec<Equation>) {
    let mut total = 0;
    let mut wrong_equations = Vec::new();
    'loop_eq: for equation in equations {
        let operators_list = make_operators_part_1((equation.numbers.len() - 1) as u32);
        for operators in operators_list {
            let mut acc = equation.numbers[0];
            for i in 0..operators.len() {
                let number = equation.numbers[i + 1];
                match operators[i] {
                    Operator::Add => acc += number,
                    Operator::Multiply => acc *= number,
                    Operator::Concatenate => {}
                };
            }
            if acc == equation.test_value {
                total += acc;
                continue 'loop_eq;
            }
        }
        wrong_equations.push(equation.clone());
    }
    (total, wrong_equations)
}

fn make_operator_list_part_2(n: usize) -> Vec<Vec<Operator>> {
    let mut operators_list = Vec::new();
    for _ in 0..n {
        operators_list = recurse(&operators_list);
    }
    operators_list
}

fn recurse(input_vector: &Vec<Vec<Operator>>) -> Vec<Vec<Operator>> {
    if input_vector.is_empty() {
        vec![
            vec![Operator::Add],
            vec![Operator::Multiply],
            vec![Operator::Concatenate],
        ]
    } else {
        let mut new_vector = Vec::new();
        for vector in input_vector {
            for operator in [Operator::Add, Operator::Multiply, Operator::Concatenate] {
                let mut cloned = vector.to_vec();
                cloned.push(operator);
                new_vector.push(cloned);
            }
        }
        new_vector
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let equations = parse(input);
    let (total, _) = get_part_1_result(&equations);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = parse(input);
    let (total_part_1, wrong_equations) = get_part_1_result(&equations);
    let mut total_part_2 = 0;
    'loop_eq: for equation in wrong_equations {
        // Now also use the concatenate operator || to try to get the test value
        let operators_list = make_operator_list_part_2(equation.numbers.len() - 1);
        for operators in operators_list {
            let mut acc = equation.numbers[0];
            for i in 0..operators.len() {
                let number = equation.numbers[i + 1];
                match operators[i] {
                    Operator::Add => acc += number,
                    Operator::Multiply => acc *= number,
                    Operator::Concatenate => {
                        let concatenated = format!("{}{}", acc, number);
                        acc = concatenated.parse().unwrap();
                    }
                };
            }
            if acc == equation.test_value {
                total_part_2 += acc;
                continue 'loop_eq;
            }
        }
    }

    Some(total_part_1 + total_part_2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
