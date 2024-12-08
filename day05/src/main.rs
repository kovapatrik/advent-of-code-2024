use std::collections::{HashMap, HashSet};

fn main() {
    let input = std::fs::read_to_string("day05/src/input.txt").unwrap();
    let manual = parse_input(&input);

    let (part1, part2) = solution(manual);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solution(manual: Manual) -> (usize, usize) {
    let mut part1 = 0;
    let mut part2 = 0;
    manual.page_updates.iter().cloned().for_each(|update| {
        if update.is_sorted_by(|a, b| manual.rules.get(b).map_or(false, |set| set.contains(a))) {
            part1 += update[update.len() / 2];
        } else {
            let mut sorted_update = update.clone();
            sorted_update.sort_by(|a, b| {
                manual
                    .rules
                    .get(b)
                    .map_or(false, |set| set.contains(a))
                    .cmp(&true)
            });
            part2 += sorted_update[sorted_update.len() / 2];
        }
    });
    (part1, part2)
}

#[derive(Debug, Clone)]
struct Manual {
    rules: HashMap<usize, HashSet<usize>>,
    page_updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Manual {
    let (rules, page_updates) = input.split_once("\n\n").unwrap();

    let mut rules_map = HashMap::new();
    for rule in rules.lines() {
        let (page_before, page) = rule.split_once("|").unwrap();
        let page_before: usize = page_before.parse().unwrap();
        let page: usize = page.parse().unwrap();

        let entry = rules_map.entry(page).or_insert(HashSet::new());
        entry.insert(page_before);
    }

    let page_updates: Vec<Vec<usize>> = page_updates
        .lines()
        .map(|line| {
            line.split(",")
                .map(|num| num.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    Manual {
        rules: rules_map,
        page_updates,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    #[test]
    fn test_part1() {
        assert_eq!(solution(parse_input(TEST_INPUT)), (143, 123));
    }
}
