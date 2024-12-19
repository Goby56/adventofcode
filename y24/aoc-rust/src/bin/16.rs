advent_of_code::solution!(16);

use advent_of_code::position::{Vec2d, Direction};
use itertools::Itertools;

struct Agent {
    pos: Vec2d,
    facing: Direction,
    dead: bool
}

fn parse_input(input: &str) -> (Vec<Vec<u32>>, Vec<Vec<char>>, Vec2d, Vec2d) {
    let mut map: Vec<Vec<char>> = vec![];
    let mut costs: Vec<Vec<u32>> = vec![];
    let mut start_pos = Vec2d {x: -1, y: -1 };
    let mut end_pos = Vec2d { x: -1, y: -1 };
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            match ch {
                'S' => { start_pos = Vec2d::from_usize(r, c) },
                'E' => { end_pos = Vec2d::from_usize(r, c) },
                _ => {}
            }
        }
        map.push(line.chars().collect());
        costs.push(vec![u32::MAX; map[r].len()]);
    }
    return (costs, map, start_pos, end_pos);
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("My solution seems to be off by 3000 (3 turns?).\nIf the output is wrong, try adding 3000.");
    let (mut costs, map, start_pos, end_pos) = parse_input(input);
    calc_maze_cost(&mut costs, &map, start_pos);
    end_pos.access_arr(&costs)
}

fn calc_maze_cost(costs: &mut Vec<Vec<u32>>, map: &Vec<Vec<char>>, start_pos: Vec2d) {
    start_pos.put_arr(costs, 0);
    let mut agents = vec![Agent {
        pos: start_pos.clone(), facing: Direction::EAST, dead: false,
    }];
    loop {
        if agents.len() == 0 {
            break;
        }
        let mut new_agents = vec![];
        let mut agents_to_remove = vec![];
        print_map(&agents, &costs, &map);
        for i in 0..agents.len() {
            if agents[i].dead {
                agents_to_remove.push(i);
                continue;
            }
            new_agents.extend(traverse(&mut agents[i], costs, &map));
        }
        for i in agents_to_remove.iter().sorted().rev() {
            agents.swap_remove(*i);
        }
        agents.extend(new_agents);
    }
}

fn traverse(agent: &mut Agent, costs: &mut Vec<Vec<u32>>, map: &Vec<Vec<char>>) -> Vec<Agent> {
    let left = agent.facing.turn(false);
    let right = agent.facing.turn(true);
    let left_pos = agent.pos.offset(&left);
    let front_pos = agent.pos.offset(&agent.facing);
    let right_pos = agent.pos.offset(&right);

    let mut new_agents = vec![];
    if let Some(new_agent) = explore_fork(agent, &left_pos, &left, costs, map) {
        new_agents.push(new_agent);
    }
    if let Some(new_agent) = explore_fork(agent, &right_pos, &right, costs, map) {
        new_agents.push(new_agent);
    }
    let ch = front_pos.access_arr(map).unwrap();
    if (ch == '.' || ch == 'E') && update_cost(&agent.pos, &front_pos, 1, costs) {
        agent.pos = front_pos;
    } else {
        agent.dead = true;
    }
    return new_agents;
}

fn explore_fork(agent: &mut Agent, fork: &Vec2d, facing: &Direction, costs: &mut Vec<Vec<u32>>, map: &Vec<Vec<char>>) -> Option<Agent> {
    let ch = fork.access_arr(map).unwrap();
    if ch == '.' || ch == 'E' {
        if update_cost(&agent.pos, &fork, 1001, costs) {
            return Some(Agent {
                pos: fork.clone(),
                facing: facing.clone(),
                dead: false
            })
        }
    }
    return None;
}

fn update_cost(prev_pos: &Vec2d, new_pos: &Vec2d, cost_update: u32, costs: &mut Vec<Vec<u32>>) -> bool {
    let curr_cost = new_pos.access_arr(costs).unwrap();
    let new_cost = prev_pos.access_arr(costs).unwrap() + cost_update;
    if new_cost < curr_cost {
        new_pos.put_arr(costs, new_cost);
        return true;
    }
    return false;
}

pub fn part_two(input: &str) -> Option<u32> {
    let (mut costs, map, start_pos, end_pos) = parse_input(input);
    let mut visited = vec![];
    for r in 0..costs.len() {
        visited.push(vec![false; costs[r].len()]);
    }
    calc_maze_cost(&mut costs, &map, start_pos);

    let mut agents = vec![
        Agent { pos: end_pos.clone(), facing: Direction::EAST, dead: false },
        Agent { pos: end_pos.clone(), facing: Direction::SOUTH, dead: false },
        Agent { pos: end_pos.clone(), facing: Direction::WEST, dead: false },
        Agent { pos: end_pos.clone(), facing: Direction::NORTH, dead: false }
    ];
    loop {
        if agents.len() == 0 {
            break;
        }
        let mut new_agents = vec![];
        let mut agents_to_remove = vec![];
        for i in 0..agents.len() {
            agents[i].pos.put_arr(&mut visited, true);
            if agents[i].dead {
                agents_to_remove.push(i);
                continue;
            }
            new_agents.extend(backtrack(&mut agents[i], &costs));
        }
        for i in agents_to_remove.iter().sorted().rev() {
            agents.swap_remove(*i);
        }
        agents.extend(new_agents);
    }
    let mut seats = 0;
    for r in 0..visited.len() {
        for c in 0..visited[0].len() {
            if visited[r][c] {
                print!("O");
                seats += 1;
            } else {
                print!("{}", map[r][c]);
            }
        }
        print!("\n");
    }
    Some(seats)
}

fn print_map(agents: &Vec<Agent>, costs: &Vec<Vec<u32>>, map: &Vec<Vec<char>>) {
    for r in 0..map.len() {
        for c in 0..map[0].len() {
            let mut agent_dir = None;
            for a in agents {
                if a.pos.x == c as i32 && a.pos.y == r as i32 {
                    agent_dir = Some(a.facing.clone());
                    break;
                }
            }
            if let Some(facing) = agent_dir {
                print!("{facing}");
            } else {
                print!("{}", map[r][c]);
            }
        }
        for c in 0..map[0].len() {
            let cost = costs[r][c];
            if map[r][c] == '#' {
                print!("{:>5} ", "#");
            } else {
                if cost == u32::MAX {
                    print!("{:>5} ", "X");
                } else {
                    print!("{:>5} ", cost);
                }

            }
        }
        print!("\n");
    }
    print!("\n");
}

fn backtrack(agent: &mut Agent, costs: &Vec<Vec<u32>>) -> Vec<Agent> {
    let left = agent.facing.turn(false);
    let right = agent.facing.turn(true);
    let left_pos = agent.pos.offset(&left);
    let front_pos = agent.pos.offset(&agent.facing);
    let right_pos = agent.pos.offset(&right);

    let mut new_agents = vec![];
    if check_cost(&agent.pos, &left_pos, costs) {
        new_agents.push(Agent {
            pos: left_pos.clone(), facing: left.clone(), dead: false
        });
    }
    if check_cost(&agent.pos, &right_pos, costs) {
        new_agents.push(Agent {
            pos: right_pos.clone(), facing: right.clone(), dead: false
        });
    }
    if check_cost(&agent.pos, &front_pos, costs) {
        agent.pos = front_pos;
    } else {
        agent.dead = true;
    }
    return new_agents;
}

fn check_cost(prev_pos: &Vec2d, new_pos: &Vec2d, costs: &Vec<Vec<u32>>) -> bool {
    let curr_cost = prev_pos.access_arr(costs).unwrap();
    let new_cost = new_pos.access_arr(costs).unwrap();
    return new_cost <= curr_cost;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7036));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
