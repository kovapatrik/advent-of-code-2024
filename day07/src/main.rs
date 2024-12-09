use itertools::{repeat_n, Itertools};

fn main() {
    let input = std::fs::read_to_string("day07/src/input.txt").unwrap();

    let calibrations = parse_input(&input);

    println!("Part 1: {}", part1(&calibrations));
    println!("Part 2: {}", part2(&calibrations));
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Operation {
    Addition,
    Multiplication,
    Concatenation,
}

fn part1(calibrations: &Vec<Calibration>) -> usize {
    let operations = vec![Operation::Addition, Operation::Multiplication];
    calibrations
        .iter()
        .filter_map(|calibration| {
            let permutations = repeat_n(operations.iter(), calibration.numbers.len() - 1)
                .multi_cartesian_product()
                .unique();

            for perm in permutations {
                let mut result = calibration.numbers[0];
                for (operation, number) in perm.iter().zip(calibration.numbers.iter().skip(1)) {
                    match operation {
                        Operation::Addition => result += number,
                        Operation::Multiplication => result *= number,
                        _ => {}
                    }
                }
                if result == calibration.test_value {
                    return Some(calibration.test_value);
                }
            }

            None
        })
        .sum()
}

fn part2(calibrations: &Vec<Calibration>) -> usize {
    let operations = vec![
        Operation::Addition,
        Operation::Multiplication,
        Operation::Concatenation,
    ];
    calibrations
        .iter()
        .filter_map(|calibration| {
            let permutations = repeat_n(operations.iter(), calibration.numbers.len() - 1)
                .multi_cartesian_product();

            for perm in permutations {
                let mut result = calibration.numbers[0];

                for (operation, number) in perm.iter().zip(calibration.numbers.iter().skip(1)) {
                    match operation {
                        Operation::Addition => result += number,
                        Operation::Multiplication => result *= number,
                        &&Operation::Concatenation => {
                            result = format!("{}{}", result, number).parse().unwrap()
                        }
                    }
                }
                if result == calibration.test_value {
                    return Some(calibration.test_value);
                }
            }

            None
        })
        .sum()
}

#[derive(Debug)]
struct Calibration {
    test_value: usize,
    numbers: Vec<usize>,
}

fn parse_input(input: &str) -> Vec<Calibration> {
    input
        .lines()
        .map(|line| {
            let (test_value, numbers) = line.split_once(": ").unwrap();
            let numbers = numbers
                .split_whitespace()
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Calibration {
                test_value: test_value.parse().unwrap(),
                numbers,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&parse_input(TEST_INPUT)), 3749);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&parse_input(TEST_INPUT)), 11387);
    }
}
