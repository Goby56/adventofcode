use std::ops::Div;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, updates) = read_rules_and_updates(input);
    let mut middle_page_sum = 0;
    for update in updates {
        if !has_rule_conflict(&update, &rules) {
            middle_page_sum += update[update.len().div(2)];
        }
    }
    Some(middle_page_sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut middle_page_sum = 0;
    let (rules, mut updates) = read_rules_and_updates(input);
    for i in 0..updates.len() {
        if was_unsorted(&mut updates[i], &rules) {
            middle_page_sum += updates[i][updates[i].len().div(2)];
        }
    }
    Some(middle_page_sum)
}

fn read_rules_and_updates(input: &str) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut rules: Vec<(u32, u32)> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];
    let mut all_rules_read = false;
    for line in input.lines() {
        if line == "" {
            all_rules_read = true;
            continue;
        }
        if all_rules_read {
            let update: Vec<u32> = line.split(',').map(|s| s.parse::<u32>().unwrap()).collect();
            updates.push(update);
        } else {
            let rule: Vec<u32> = line.splitn(2, '|').map(|s| s.parse::<u32>().unwrap()).collect();
            rules.push((rule[0], rule[1]))
        }
    }
    return (rules, updates)
}

fn was_unsorted(update: &mut Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    let mut sorted = false;
    let mut was_unsorted = false;
    while !sorted {
        sorted = true;
        for rule in rules {
            let before = rule.0;
            let after = rule.1;
            let mut before_index = None;
            let mut after_index = None;
            for i in 0..update.len() {
                if update[i] == before {
                    before_index = Some(i);
                }
                if update[i] == after {
                    after_index = Some(i);
                }
            }
            if let Some(bi) = before_index {
                if let Some(ai) = after_index {
                    if bi > ai {
                        let temp = update[bi];
                        update[bi] = update[ai];
                        update[ai] = temp;
                        sorted = false;
                        was_unsorted = true;
                    }
                }
            }
        }
    }
    return was_unsorted;
}

fn has_rule_conflict(update: &Vec<u32>, rules: &Vec<(u32, u32)>) -> bool {
    for rule in rules {
        let before = rule.0;
        let after = rule.1;
        let mut before_index = None;
        let mut after_index = None;
        for i in 0..update.len() {
            if update[i] == before {
                before_index = Some(i);
            }
            if update[i] == after {
                after_index = Some(i);
            }
        }
        if let Some(bi) = before_index {
            if let Some(ai) = after_index {
                if bi > ai {
                    return true;
                }
            }
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
