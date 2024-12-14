use std::usize;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut pos = (0i32, 0i32);
    let mut look_dir = '?';
    let mut obstacles: Vec<Vec<bool>> = vec![];
    let mut visited_positions: Vec<Vec<bool>> = vec![];
    for (r, line) in input.lines().enumerate() {
        visited_positions.push((0..line.len()).map(|_| false).collect());
        for (c, char) in line.chars().enumerate() {
            if [">", "<", "^", "v"].contains(&char.to_string().as_str()) {
                look_dir = char;
                pos = (r as i32, c as i32);
            }
        }
        obstacles.push(line.chars().map(|c| c == '#').collect());
    }
    loop {
        visited_positions[pos.0 as usize][pos.1 as usize] = true;
        let (dr, dc) = walk_offset(look_dir);
        let r = pos.0 + dr;
        let c = pos.1 + dc;
        if r < 0 || r >= obstacles.len() as i32 || c < 0 || c >= obstacles[0].len() as i32 {
            break;
        }
        if obstacles[r as usize][c as usize] {
            look_dir = turn(look_dir);
            continue;
        }
        pos.0 = r;
        pos.1 = c;
    }
    Some(visited_positions.iter()
        .map(|row| row.iter().fold(0, |sum, visited| sum + if *visited {1} else {0}))
        .fold(0, |sum, row_sum| sum + row_sum))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut start_pos = (0i32, 0i32);
    let mut obstacles: Vec<Vec<bool>> = vec![];
    let mut start_look_dir = '?';
    let mut visited_positions: Vec<Vec<char>> = vec![];
    for (r, line) in input.lines().enumerate() {
        visited_positions.push(line.chars().collect());
        for (c, char) in line.chars().enumerate() {
            if [">", "<", "^", "v"].contains(&char.to_string().as_str()) {
                start_pos = (r as i32, c as i32);
                start_look_dir = char;
            }
        }
        obstacles.push(line.chars().map(|c| c == '#').collect());
    }

    let mut loop_count = 0;
    for r in 0..obstacles.len() {
        for c in 0..obstacles[0].len() {
            if obstacles[r][c] || (r as i32 == start_pos.0 && c as i32 == start_pos.1) {
                continue;
            }
            obstacles[r][c] = true;
            if has_loop(&obstacles, start_pos, start_look_dir) {
                loop_count += 1;
            }
            obstacles[r][c] = false;
        }
    }
    Some(loop_count)
}

fn walk_offset(look_dir: char) -> (i32, i32) {
    match look_dir {
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        '^' => (-1, 0),
        _ => (0, 0)
    }
}

fn turn(look_dir: char) -> char {
    let directions = ['>', 'v', '<', '^'];
    for i in 0..4 {
        if directions[i] == look_dir {
            return directions[(i+1)%4];
        }
    }
    look_dir
}


fn has_loop(obstacles: &Vec<Vec<bool>>, start_pos: (i32, i32), start_look_dir: char) -> bool {
    let mut direction_field: Vec<Vec<char>> = obstacles.iter().map(|v| v.iter().map(|_| ' ').collect()).collect();
    let mut pos = start_pos;
    let mut look_dir = start_look_dir;
    loop {
        if direction_field[pos.0 as usize][pos.1 as usize] == look_dir {
            break true;
        }
        let (dr, dc) = walk_offset(look_dir);
        let r = pos.0 + dr;
        let c = pos.1 + dc;
        if r < 0 || r >= obstacles.len() as i32 || c < 0 || c >= obstacles[0].len() as i32 {
            break false;
        }
        if obstacles[r as usize][c as usize] {
            look_dir = turn(look_dir);
            continue;
        }
        direction_field[pos.0 as usize][pos.1 as usize] = look_dir;
        pos.0 = r;
        pos.1 = c;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
