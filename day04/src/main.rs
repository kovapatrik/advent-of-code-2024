fn main() {
    let input = std::fs::read_to_string("day04/src/input.txt").unwrap();

    println!("Part 1: {}", part1(parse_input(&input)));
    println!("Part 2: {}", part2(parse_input(&input)));
}

type WordSearch = Vec<Vec<char>>;

const DIRECTIONS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
];

fn part1(word_search: WordSearch) -> usize {
    let mut count = 0;
    let word = "XMAS";
    for (i, row) in word_search.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'X' {
                let i = i as isize;
                let j = j as isize;
                for (di, dj) in DIRECTIONS.iter() {
                    if (i + di * (word.len() as isize - 1)) < 0
                        || (i + di * (word.len() as isize - 1)) >= (word_search.len() as isize)
                        || (j + dj * (word.len() as isize - 1)) < 0
                        || (j + dj * (word.len() as isize - 1)) >= (word_search[0].len() as isize)
                    {
                        continue;
                    }
                    for l in 1..word.chars().count() {
                        if word_search[(i + di * l as isize) as usize]
                            [(j + dj * l as isize) as usize]
                            != word.chars().nth(l).unwrap()
                        {
                            break;
                        }
                        if l == word.chars().count() - 1 {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    count
}

fn part2(word_search: WordSearch) -> usize {
    let mut count = 0;
    for (i, row) in word_search.iter().enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'A' {
                if j.checked_sub(1).is_none()
                    || j + 1 >= word_search[0].len()
                    || i.checked_sub(1).is_none()
                    || i + 1 >= word_search.len()
                {
                    continue;
                }
                if (word_search[i - 1][j - 1] == 'M' && word_search[i + 1][j + 1] == 'S'
                    || word_search[i - 1][j - 1] == 'S' && word_search[i + 1][j + 1] == 'M')
                    && (word_search[i - 1][j + 1] == 'M' && word_search[i + 1][j - 1] == 'S'
                        || word_search[i - 1][j + 1] == 'S' && word_search[i + 1][j - 1] == 'M')
                {
                    count += 1;
                }
            }
        }
    }
    count
}

fn parse_input(input: &str) -> WordSearch {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(parse_input(TEST_INPUT)), 18);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(parse_input(TEST_INPUT)), 9);
    }
}
