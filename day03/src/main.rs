use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("day03/src/input.txt").unwrap();

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

const MUL_REGEXP: &str = r"mul\((\d+),(\d+)\)";
const MUL_DO_DONT_REGEXP: &str = r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)";

fn part1(input: &str) -> usize {
    let re = Regex::new(MUL_REGEXP).unwrap();

    let result: usize = re
        .captures_iter(input)
        .map(|cap| {
            let mul_first = cap[1].parse::<usize>().unwrap();
            let mul_second = cap[2].parse::<usize>().unwrap();
            mul_first * mul_second
        })
        .sum();

    result
}

fn part2(input: &str) -> usize {
    let re = Regex::new(MUL_DO_DONT_REGEXP).unwrap();

    let mut can_do = true;

    let result: usize = re
        .captures_iter(input)
        .map(|cap| {
            let mut is_mult = false;
            match &cap[0] {
                "do()" => can_do = true,
                "don't()" => can_do = false,
                _ => is_mult = true,
            }
            if can_do && is_mult {
                let mul_first = cap[1].parse::<usize>().unwrap();
                let mul_second = cap[2].parse::<usize>().unwrap();
                mul_first * mul_second
            } else {
                0
            }
        })
        .sum();

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 161);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 48);
    }
}
