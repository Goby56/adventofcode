use std::usize;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut map, moves) = read_input(input);
    let mut pos = (0, 0);
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '@' {
                pos = (r, c);
            }
        }
    }
    for direction in moves {
        pos = move_on_map(&mut map, pos, direction);
    }
    let mut gps = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == 'O' {
                gps += r * 100 + c;
            }
        }
    }
    Some(gps as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn get_next(curr_pos: (usize, usize), direction: char) -> (usize, usize) {
    let offset = match direction {
        '>' => Some((0, 1)),
        'v' => Some((1, 0)),
        '<' => Some((0, -1)),
        '^' => Some((-1, 0)),
        _ => None
    }.unwrap();
    ((curr_pos.0 as i32 + offset.0) as usize, (curr_pos.1 as i32 + offset.1) as usize)
}

fn dist_to_wall(map_dim: (usize, usize), pos: (usize, usize), direction: char) -> Option<usize> {
    match direction {
        '>' => Some(map_dim.1 - pos.1),
        'v' => Some(map_dim.0 - pos.0),
        '<' => Some(pos.1),
        '^' => Some(pos.0),
        _ => None
    }
}

fn move_on_map(map: &mut Vec<Vec<char>>, pos: (usize, usize), direction: char) -> (usize, usize) {
    let n = dist_to_wall((map.len(), map[0].len()), pos, direction).unwrap();
    let mut can_push = false;
    let (mut r, mut c) = pos;
    for _ in 1..n {
        (r, c) = get_next((r, c), direction);
        if map[r][c] == '.' {
            can_push = true;
            break;
        } else if map[r][c] == '#' {
            break;
        }
    }
    if !can_push {
        return pos;
    }
    let (mut r1, mut c1) = pos;
    for _ in 1..n {
        let (r2, c2) = get_next((r1, c1), direction);
        if map[r1][c1] == 'O' && map[r2][c2] == '.' {
            map[r2][c2] = 'O';
            break;
        } else if map[r1][c1] == '@' && map[r2][c2] == '.' {
            map[r1][c1] = '.';
            map[r2][c2] = '@';
            break;
        }
        r1 = r2;
        c1 = c2;
    }
    let (r, c) = get_next(pos, direction);
    map[pos.0][pos.1] = '.';
    map[r][c] = '@';
    return (r, c);
}

fn read_input(input: &str) -> (Vec<Vec<char>>, Vec<char>) {
    let mut map: Vec<Vec<char>> = vec![];
    let mut moves: Vec<char> = vec![];
    let mut map_read = false;
    for line in input.lines() {
        if line == "" {
            map_read = true;
            continue;
        }
        if map_read {
            moves.extend(line.chars());
        } else {
            map.push(line.chars().collect());
        }
    }
    (map, moves)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
