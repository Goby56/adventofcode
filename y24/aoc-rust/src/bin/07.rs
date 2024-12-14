use itertools::Itertools;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result_sum = 0;
    for line in input.lines() {
        let mut equation = line.split(' ').map(|s| s.replace(':', "").parse::<u64>().unwrap());
        let result = equation.next().unwrap();
        let numbers: Vec<u64> = equation.collect();

        let tot_combinations = 2u64.pow((numbers.len()-1) as u32);
        for comb in 0..tot_combinations {
            let mut res = numbers[0];
            for i in 0..(numbers.len()-1) {
                if res > result {
                    break;
                }
                if (comb >> i & 1) == 1 {
                    res *= numbers[i+1];
                } else {
                    res += numbers[i+1];
                }
            }
            if res == result {
                result_sum += result;
                break;
            }
        }
    }
    Some(result_sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result_sum = 0;
    for line in input.lines() {
        let mut equation = line.split(' ').map(|s| s.replace(':', "").parse::<u64>().unwrap());
        let result = equation.next().unwrap();
        let numbers: Vec<u64> = equation.collect();

        let n = numbers.len() - 1;
        for comb in vec![0..3; n].into_iter().multi_cartesian_product() {
            let mut res = numbers[0];
            for i in 0..n {
                if res > result {
                    break;
                }
                res = match comb[i] {
                    1 => res * numbers[i+1],
                    2 => res + numbers[i+1],
                    _ => (res.to_string() + (&numbers[i+1].to_string())).parse::<u64>().unwrap()
                }
            }
            if res == result {
                result_sum += result;
                break;
            }
        }
    }
    Some(result_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
