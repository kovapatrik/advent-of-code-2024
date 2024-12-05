use std::{collections::HashMap, ops::Index};

fn main() {
    let input = std::fs::read_to_string("day05/src/input.txt").unwrap();
    let manual = parse_input(&input);
    println!("Part 1: {}", part1(manual));
}

fn create_update_from_rules(
    rules: HashMap<usize, Vec<usize>>,
    numbers_of_page: Vec<usize>,
) -> Vec<usize> {
    let mut page_update = Vec::new();
    for num in numbers_of_page.iter() {
        println!("{:?} {:?}", num, page_update);
        match rules.get(num) {
            Some(pages_before) => {
                let valid_pages_before: Vec<usize> = pages_before
                    .iter()
                    .cloned()
                    .filter(|page| numbers_of_page.contains(page))
                    .collect();
                println!("{:?}", valid_pages_before);
                page_update = page_update
                    .iter()
                    .cloned()
                    .filter(|page| !valid_pages_before.contains(page))
                    .collect();
                page_update.extend(valid_pages_before);
            }
            None => (),
        }
        page_update.push(*num);
    }
    page_update
}

fn is_valid_page_update(rules: HashMap<usize, Vec<usize>>, update: Vec<usize>) -> bool {
    for (i, page_number) in update.iter().cloned().enumerate() {
        match rules.get(&page_number) {
            Some(pages_before) => {
                if update[..i]
                    .iter()
                    .any(|page_before| !pages_before.contains(page_before))
                    || update[i + 1..]
                        .iter()
                        .any(|page_before| pages_before.contains(page_before))
                {
                    return false;
                }
            }
            None => {
                continue;
            }
        }
    }
    true
}

fn part1(manual: Manual) -> usize {
    manual
        .page_updates
        .iter()
        .cloned()
        .filter(|update| is_valid_page_update(manual.rules.clone(), update.clone()))
        .map(|valid_update| valid_update[valid_update.len() / 2])
        .sum()
}

fn part2(manual: Manual) -> usize {
    manual
        .page_updates
        .iter()
        .cloned()
        .filter(|update| !is_valid_page_update(manual.rules.clone(), update.clone()))
        .map(|invalid_update| {
            let mut fixed_update = Vec::new();

            for i in 0..fixed_update.len() {
                let page_number = fixed_update[i];
                match manual.rules.get(&page_number) {
                    Some(pages_before) => {
                        pages_before
                            .iter()
                            .cloned()
                            .filter(|page_before| {
                                !invalid_update[..i].contains(page_before)
                                    && invalid_update[i + 1..].contains(page_before)
                            })
                            .for_each(|page_before| {
                                fixed_update[i] = page_before;
                            });
                    }
                    None => {
                        fixed_update.push(page_number);
                    }
                }
            }

            println!("{:?} -> {:?}", invalid_update, fixed_update);

            fixed_update[fixed_update.len() / 2]
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Manual {
    rules: HashMap<usize, Vec<usize>>,
    page_updates: Vec<Vec<usize>>,
}

fn parse_input(input: &str) -> Manual {
    let (rules, page_updates) = input.split_once("\n\n").unwrap();

    let mut rules_map = HashMap::new();
    for rule in rules.lines() {
        let (page_before, page) = rule.split_once("|").unwrap();
        let page_before: usize = page_before.parse().unwrap();
        let page: usize = page.parse().unwrap();

        let entry = rules_map.entry(page).or_insert(Vec::new());
        entry.push(page_before);
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
        assert_eq!(part1(parse_input(TEST_INPUT)), 143);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(TEST_INPUT)), 123);
    }

    #[test]
    fn test_create_update_from_rules() {
        let manual = parse_input(TEST_INPUT);
        assert_eq!(
            create_update_from_rules(manual.rules.clone(), manual.page_updates[0].clone()),
            vec![47, 53]
        );
    }
}
