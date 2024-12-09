use std::collections::{HashMap, HashSet};

use itertools::Itertools as _;

fn main() {
    let input = std::fs::read_to_string("day08/src/input.txt").unwrap();

    let (antenna_type_positions, map_height, map_width) = parse_input(&input);

    println!(
        "Part 1: {}",
        part1(&antenna_type_positions, map_height, map_width)
    );
    println!(
        "Part 2: {}",
        part2(&antenna_type_positions, map_height, map_width)
    );
}

fn find_antinodes(
    antenna_type_positions: &AntennaTypePositions,
    map_height: usize,
    map_width: usize,
) -> HashSet<(usize, usize)> {
    let mut antinodes = HashSet::new();

    for (_, positions) in antenna_type_positions {
        if positions.len() < 2 {
            continue;
        }

        positions.iter().combinations(2).for_each(|comb| {
            let (p1, p2) = comb[0].clone();
            let (q1, q2) = comb[1].clone();

            let p1 = p1 as isize;
            let p2 = p2 as isize;
            let q1 = q1 as isize;
            let q2 = q2 as isize;

            let p1_extended = p1 + (p1 - q1);
            let p2_extended = p2 + (p2 - q2);
            let q1_extended = q1 + (q1 - p1);
            let q2_extended = q2 + (q2 - p2);

            if p1_extended >= 0
                && p1_extended < map_width as isize
                && p2_extended >= 0
                && p2_extended < map_height as isize
            {
                antinodes.insert((p1_extended as usize, p2_extended as usize));
            }

            if q1_extended >= 0
                && q1_extended < map_width as isize
                && q2_extended >= 0
                && q2_extended < map_height as isize
            {
                antinodes.insert((q1_extended as usize, q2_extended as usize));
            }
        });
    }

    antinodes
}

fn part1(
    antenna_type_positions: &AntennaTypePositions,
    map_height: usize,
    map_width: usize,
) -> usize {
    let anti_nodes = find_antinodes(antenna_type_positions, map_height, map_width);
    anti_nodes.len()
}

fn part2(
    antenna_type_positions: &AntennaTypePositions,
    map_height: usize,
    map_width: usize,
) -> usize {
    let mut antinodes = HashSet::new();

    for (_, positions) in antenna_type_positions {
        if positions.len() < 2 {
            continue;
        }

        antinodes.extend(positions.iter().cloned());

        positions.iter().combinations(2).for_each(|comb| {
            let (p1, p2) = comb[0].clone();
            let (q1, q2) = comb[1].clone();

            let mut p1 = p1 as isize;
            let mut p2 = p2 as isize;
            let mut q1 = q1 as isize;
            let mut q2 = q2 as isize;

            let p1_diff = p1 - q1;
            let p2_diff = p2 - q2;
            let q1_diff = q1 - p1;
            let q2_diff = q2 - p2;

            while p1 + p1_diff >= 0
                && p1 + p1_diff < map_width as isize
                && p2 + p2_diff >= 0
                && p2 + p2_diff < map_height as isize
            {
                let new_p1 = (p1 + p1_diff) as usize;
                let new_p2 = (p2 + p2_diff) as usize;
                antinodes.insert((new_p1, new_p2));
                p1 = new_p1 as isize;
                p2 = new_p2 as isize;
            }

            while q1 + q1_diff >= 0
                && q1 + q1_diff < map_width as isize
                && q2 + q2_diff >= 0
                && q2 + q2_diff < map_height as isize
            {
                let new_q1 = (q1 + q1_diff) as usize;
                let new_q2 = (q2 + q2_diff) as usize;
                antinodes.insert((new_q1, new_q2));
                q1 = new_q1 as isize;
                q2 = new_q2 as isize;
            }
        });
    }

    antinodes.len()
}

type AntennaTypePositions = HashMap<char, Vec<(usize, usize)>>;

fn parse_input(input: &str) -> (AntennaTypePositions, usize, usize) {
    let mut antenna_type_positions = AntennaTypePositions::new();

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| {
            if c != '.' {
                antenna_type_positions
                    .entry(c)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        });
    });

    (
        antenna_type_positions,
        input.lines().count(),
        input.lines().next().unwrap().len(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

    #[test]
    fn test_part1() {
        let (antenna_type_positions, map_height, map_width) = parse_input(TEST_INPUT);
        assert_eq!(part1(&antenna_type_positions, map_height, map_width), 14);
    }

    #[test]
    fn test_part2() {
        let (antenna_type_positions, map_height, map_width) = parse_input(TEST_INPUT);
        assert_eq!(part2(&antenna_type_positions, map_height, map_width), 34);
    }
}
