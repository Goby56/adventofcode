use std::collections::HashMap;

advent_of_code::solution!(8);

fn parse_input(input: &str) -> (Vec<Vec<char>>, HashMap<char, Vec<(i32, i32)>>) {
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut map: Vec<Vec<char>> = vec![];
    for (r, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (c, char) in line.chars().enumerate() {
            row.push(char);
            if char != '.' {
                if let Some(antenna_positions) = antennas.get_mut(&char) {
                    antenna_positions.push((r as i32, c as i32));
                } else {
                    antennas.insert(char, vec![(r as i32, c as i32)]);
                }
            }
        }
        map.push(row);
    }
    return (map, antennas);
}

pub fn part_one(input: &str) -> Option<u32> {
    let (map, antennas) = parse_input(input);
    let mut anti_nodes = 0;
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if is_anti_node(r as i32, c as i32, &antennas) {
                anti_nodes += 1;
            }
        }
    }
    Some(anti_nodes)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut map, antennas) = parse_input(input);
    let mut anti_nodes = 0;
    for positions in antennas.values() {
        for p1 in positions {
            for p2 in positions {
                if p1.0 == p2.0 && p1.1 == p2.1 {
                    continue;
                }
                spread_antinodes(*p1, *p2, &mut map);
            }
        }
    }
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            if map[r][c] == '#' {
                anti_nodes += 1;
            }
        }
    }
    Some(anti_nodes)
}

fn spread_antinodes(p1: (i32, i32), p2: (i32, i32), map: &mut Vec<Vec<char>>) {
    let direction_vec = (p1.0 - p2.0, p1.1 - p2.1);
    let n = map.len() as i32;
    let m = map[0].len() as i32;
    for t in (-n*m)..(n*m) {
        let (r, c) = (p1.0 + direction_vec.0 * t, p1.1 + direction_vec.1 * t);
        if r < 0 || r > n - 1 || c < 0 || c > m - 1 {
            continue;
        }
        map[r as usize][c as usize] = '#';
    }
}

fn is_anti_node(r: i32, c: i32, antennas: &HashMap<char, Vec<(i32, i32)>>) -> bool {
    for positions in antennas.values() {
        for p1 in positions {
            for p2 in positions {
                if p1.0 == p2.0 && p1.1 == p2.1 {
                    continue;
                }
                if double_distance(r, c, *p1, *p2) {
                    return true;
                }
            }
        }
    }
    return false;
}

fn double_distance(r: i32, c: i32, p1: (i32, i32), p2: (i32, i32)) -> bool {
    r - p1.0 == 2 * (r - p2.0) && c - p1.1 == 2 * (c - p2.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
