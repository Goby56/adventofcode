use std::usize;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let puzzle: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut xmases = 0;
    for r in 0..puzzle.len() {
        for c in 0..puzzle[0].len() {
            if puzzle[r][c] == 'X' {
                xmases += search_axes(r, c, &puzzle);
            }
        }
    }
    Some(xmases)
}

pub fn part_two(input: &str) -> Option<u32> {
    let puzzle: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();
    None
}

pub fn search_axes(row: usize, column: usize, puzzle: &Vec<Vec<char>>) -> u32 {
    let mut xmases = 0;
    let mut cache: Vec<char> = vec![];
    let dim = (puzzle.len(), puzzle[0].len());
    for i in -1..2 {
        for j in -1..2 {
            if i == 0 && j == 0 { continue; }
            cache.clear();
            for offset in directed_range(i, j, dim.0.max(dim.1)) {
                let r = row as i32 + offset.0;
                let c = column as i32 + offset.1;
                if r < 0 || c < 0 || r >= dim.0 as i32 || c >= dim.1 as i32 {
                    break;
                }
                let char = puzzle[r as usize][c as usize];
                if char == 'X' {
                    break;
                }
                if !correct_order(char, &mut cache) {
                    break;
                }
                if cache.len() == 3 {
                    xmases += 1;
                    break;
                }
            }
        }
    }
    return xmases;
}

pub fn directed_range(dr: i32, dc: i32, len: usize) -> Vec<(i32, i32)> {
    (1..(len as i32)).map(|n| (n * dr, n * dc)).collect()
}

pub fn correct_order(c: char, cache: &mut Vec<char>) -> bool {
    let should_push = match (cache.last(), c) {
        (None, 'M') => {
            true
        },
        (Some('M'), 'A') => {
            true
        },
        (Some('A'), 'S') => {
            true
        },
        _ => false
    };
    if should_push {
        cache.push(c);
        return true;
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
