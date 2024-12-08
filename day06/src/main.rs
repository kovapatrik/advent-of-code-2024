use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("day06/src/input.txt").unwrap();
    let map = parse_input(&input);

    let (part1, part2) = solution(map);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn walk(input: &Map, start_point: (usize, usize), is_part1: bool) -> Option<Vec<(usize, usize)>> {
    let mut current_point = start_point;
    let mut current_direction = Direction::Up;
    let mut visited_points_with_directions = HashSet::new();

    loop {
        if !visited_points_with_directions.insert((current_point, current_direction)) {
            return None;
        }

        let (x, y) = current_point;
        let (dx, dy) = match current_direction {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        if x as isize + dx < 0
            || y as isize + dy < 0
            || y as isize + dy >= input.len() as isize
            || x as isize + dx >= input[y as usize].len() as isize
        {
            if !is_part1 {
                return Some(Vec::new());
            }

            let res: HashSet<(usize, usize)> = visited_points_with_directions
                .iter()
                .map(|(point, _)| *point)
                .collect();

            return Some(res.into_iter().collect());
        }

        let next_point = (x as isize + dx, y as isize + dy);
        let next_cell = input[next_point.1 as usize][next_point.0 as usize];

        if next_cell == '#' {
            current_direction = match current_direction {
                Direction::Up => Direction::Right,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
                Direction::Right => Direction::Down,
            };
        } else {
            current_point = (next_point.0 as usize, next_point.1 as usize);
        }
    }
}

fn solution(mut input: Map) -> (usize, usize) {
    let start_point = input
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, &cell)| if cell == '^' { Some((x, y)) } else { None })
        })
        .unwrap();

    let part1 = walk(&input, start_point, true).unwrap();
    let part2 = part1
        .iter()
        .filter(|point| {
            input[point.1][point.0] = '#';
            let ok = walk(&input, start_point, false).is_none();
            input[point.1][point.0] = 'X';
            ok
        })
        .count();

    (part1.len(), part2)
}

fn parse_input(input: &str) -> Map {
    input.lines().map(|line| line.chars().collect()).collect()
}

type Map = Vec<Vec<char>>;

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    #[test]
    fn test_solution() {
        let map = parse_input(TEST_INPUT);
        assert_eq!(solution(map), (41, 6));
    }
}
