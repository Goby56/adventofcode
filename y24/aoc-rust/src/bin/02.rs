advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    for line in input.lines() {
        let lvls: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        if is_safe_report(lvls) {
            safe_reports += 1;
        }
    }
    Some(safe_reports)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut safe_reports = 0;
    for line in input.lines() {
        let lvls: Vec<u32> = line.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect();
        for i in 0..lvls.len() {
            let without_i: Vec<u32> = lvls.iter().enumerate()
                .filter(|(j, _)| i != *j)
                .map(|(_, l)| *l).collect();
            if is_safe_report(without_i) {
                safe_reports += 1;
                break;
            }
        }
    }
    Some(safe_reports)
}

pub fn is_safe_report(lvls: Vec<u32>) -> bool {
    let mut is_safe = true;
    let mut increments = 0;
    for i in 1..lvls.len() {
        let a = lvls[i-1];
        let b = lvls[i];
        if a == b || !(1..4).contains(&a.abs_diff(b)) {
            is_safe = false;
            break
        }
        if a < b {
            increments += 1;
        }
    }
    is_safe && (increments == 0 || increments == lvls.len() - 1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
