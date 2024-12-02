#[derive(Clone)]
struct UnusualData {
    reports: Vec<Report>,
}

#[derive(Clone)]
struct Report {
    levels: Vec<usize>,
}

fn main() {
    let input = std::fs::read_to_string("day02/src/input.txt").unwrap();

    let unusual_data = parse_input(&input);

    println!("Part 1: {}", part1(unusual_data.clone()));
    println!("Part 2: {}", part2(unusual_data.clone()));
}

fn is_safe(report: Report) -> bool {
    let first_diff = report.levels[1] as isize - report.levels[0] as isize;
    report
        .levels
        .iter()
        .zip(report.levels.iter().skip(1))
        .all(|(a, b)| {
            let diff = *b as isize - *a as isize;
            (first_diff > 0 && diff >= 1 && diff <= 3)
                || (first_diff < 0 && diff <= -1 && diff >= -3)
        })
}

fn part1(unusual_data: UnusualData) -> usize {
    unusual_data
        .reports
        .iter()
        .filter(|report| is_safe((*report).clone()))
        .count()
}

fn is_safe2(report: Report) -> bool {
    if is_safe(report.clone()) {
        return true;
    }

    for i in 0..report.levels.len() {
        let mut new_report = report.clone();
        new_report.levels.remove(i);
        if is_safe(new_report) {
            return true;
        }
    }

    false
}

fn part2(unusual_data: UnusualData) -> usize {
    unusual_data
        .reports
        .iter()
        .filter(|report| is_safe2((*report).clone()))
        .count()
}

fn parse_input(input: &str) -> UnusualData {
    let reports = input
        .lines()
        .map(|line| {
            let levels = line
                .split_whitespace()
                .map(|level| level.parse().unwrap())
                .collect();
            Report { levels }
        })
        .collect();

    UnusualData { reports }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -1 -2 -2 -1
    // 1 5 1 1
    // -2 -1 -4 -1
    // 2 -1 2 1

    const TEST_INPUT: &str = r#"7 6 4 2 1
    1 2 7 8 9
    9 7 6 2 1
    1 3 2 4 5
    8 6 4 4 1
    1 3 6 7 9"#;

    #[test]
    fn test_part1() {
        let unusual_data = parse_input(TEST_INPUT);
        assert_eq!(part1(unusual_data), 2);
    }

    #[test]
    fn test_part2() {
        let unusual_data = parse_input(TEST_INPUT);
        assert_eq!(part2(unusual_data), 4);
    }
}
