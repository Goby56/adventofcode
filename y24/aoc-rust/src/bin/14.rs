use std::{io, ops::Div};

use itertools::Itertools;

advent_of_code::solution!(14);

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

struct Robot {
    pos: (i32, i32), // x, y
    vel: (i32, i32)
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut robots = get_robots(input);
    for _ in 0..100 {
        for r in &mut robots {
            r.pos.0 = (r.pos.0 + r.vel.0).rem_euclid(WIDTH);
            r.pos.1 = (r.pos.1 + r.vel.1).rem_euclid(HEIGHT);
        }
    }
    let mut quadrant_count = vec![0, 0, 0, 0];
    for r in robots {
        if r.pos.0 > WIDTH / 2 && r.pos.1 < HEIGHT / 2 {
            quadrant_count[0] += 1;
        }
        if r.pos.0 < WIDTH / 2 && r.pos.1 < HEIGHT / 2 {
            quadrant_count[1] += 1;
        }
        if r.pos.0 < WIDTH / 2 && r.pos.1 > HEIGHT / 2 {
            quadrant_count[2] += 1;
        }
        if r.pos.0 > WIDTH / 2 && r.pos.1 > HEIGHT / 2 {
            quadrant_count[3] += 1;
        }
    }
    Some(quadrant_count.iter().product())
}

fn part_two_solved(input: &str) -> Option<u32> {
    let mut robots = get_robots(input);
    let seconds = 7371;
    for _ in 0..seconds {
        for r in &mut robots {
            r.pos.0 = (r.pos.0 + r.vel.0).rem_euclid(WIDTH);
            r.pos.1 = (r.pos.1 + r.vel.1).rem_euclid(HEIGHT);
        }
    }
    print_room(&robots);
    Some(seconds)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut input_string = String::new();
    println!("Skip to solution? (y/N)");
    io::stdin().read_line(&mut input_string).unwrap();
    if input_string.trim() == "y" {
        return part_two_solved(input);
    }
    let mut robots = get_robots(input);
    let n = 5;
    for i in 0..10000 {
        let mut sample_areas = vec![0; n*n];
        for r in &mut robots {
            r.pos.0 = (r.pos.0 + r.vel.0).rem_euclid(WIDTH);
            r.pos.1 = (r.pos.1 + r.vel.1).rem_euclid(HEIGHT);
            let row = (n as f32 * r.pos.0 as f32).div(WIDTH as f32) as usize;
            let col = (n as f32 * r.pos.1 as f32).div(HEIGHT as f32) as usize;
            sample_areas[row * n + col] += 1;
        }
        for robot_count in sample_areas {
            if robot_count > 2 * robots.len() / (n*n) {
                print_room(&robots);
                println!("{i}");
                input_string.clear();
                io::stdin().read_line(&mut input_string).unwrap();
                if input_string.trim() == "q" {
                    return Some(i);
                }
                break;
            }
        }
    }
    None
}

fn print_room(robots: &Vec<Robot>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let mut has_robot = false;
            for r in robots {
                if r.pos.0 == x && r.pos.1 == y {
                    has_robot = true;
                    break;
                }
            }
            if has_robot {
                print!("#");
            } else {
                print!(" ");
            }
        }
        print!("\n");
    }
}

fn get_robots(input: &str) -> Vec<Robot> {
    let mut robots = vec![];
    for line in input.lines() {
        let values: Vec<Vec<i32>> = line.splitn(2, " ")
            .map(|s| s.chars()
                .skip(2)
                .collect::<String>()
                .splitn(2, ",")
                .map(|n| n.parse::<i32>().unwrap())
                .collect())
            .collect();
        robots.push(Robot{pos: (values[0][0], values[0][1]), vel: (values[1][0], values[1][1])});
    }
    return robots;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
