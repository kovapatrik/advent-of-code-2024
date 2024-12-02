struct UnusualData {
    reports: Vec<Report>,
}

struct Report {
    levels: Vec<usize>,
}

fn main() {
    let input = std::fs::read_to_string("day02/src/input.txt").unwrap();

    let unusual_data = parse_input(&input);

    println!("Part 1: {}", part1(unusual_data));
}

fn part1(unusual_data: UnusualData) -> usize {
    unusual_data
        .reports
        .iter()
        .filter(|report| {
            let mut increasing = true;
            let mut decreasing = true;
            let mut unusual = false;
            report
                .levels
                .iter()
                .zip(report.levels.iter().skip(1))
                .for_each(|(a, b)| {
                    if a > b {
                        increasing = false;
                    } else if a < b {
                        decreasing = false;
                    }

                    if increasing == false && decreasing == false {
                        unusual = true;
                        return;
                    }

                    let diff = a.abs_diff(*b);
                    if diff < 1 || diff > 3 {
                        unusual = true;
                        return;
                    }
                });

            return !unusual;
        })
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
