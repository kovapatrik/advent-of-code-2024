use std::collections::HashMap;

fn part1(mut left_list: Vec<u32>, mut right_list: Vec<u32>) -> u32 {

    left_list.sort();
    right_list.sort();

    let result = left_list.iter().zip(right_list.iter())
        .fold(0,|acc, (left, right)| acc + left.abs_diff(*right));

    result
}

fn part2(left_list: Vec<u32>, right_list: Vec<u32>) -> u32 {

    let mut occurance_map: HashMap<u32, u32> = HashMap::new();
    
    for id in right_list {
        let count = occurance_map.entry(id).or_insert(0);
        *count += 1;
    }

    let result = left_list.iter()
        .fold(0, |acc, id| acc + (id * occurance_map.get(id).unwrap_or(&0)));
    
    result
}

fn main() {
    let input = std::fs::read_to_string("day01/src/input.txt").unwrap();

    let mut left_list: Vec<u32> = Vec::new();
    let mut right_list: Vec<u32> = Vec::new();

    for line in input.lines() {
        let splitted: Vec<&str> = line.split_whitespace().collect();
        left_list.push(splitted[0].parse().unwrap());
        right_list.push(splitted[1].parse().unwrap());
    }

    println!("Part 1: {}", part1(left_list.clone(), right_list.clone()));
    println!("Part 2: {}", part2(left_list, right_list));
}
