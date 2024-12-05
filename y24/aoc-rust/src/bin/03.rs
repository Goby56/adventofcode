use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(capture_and_sum(input))
}

pub fn capture_and_sum(str: &str) -> u32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(str).map(|capture| capture.extract())
        .map(|(_, [a, b])| (a.parse::<u32>().unwrap_or(0), b.parse::<u32>().unwrap_or(0)))
        .fold(0, |sum, (a, b)| sum + a*b)
}

pub fn part_two(input: &str) -> Option<u32> {
    let re = Regex::new(r"do\(\)|don't\(\)|mul\(\d{1,3},\d{1,3}\)").unwrap();
    let mut mul_enabled = true;
    let mut sum = 0;
    for m in re.find_iter(input) {
        match m.as_str() {
            "do()" => { mul_enabled = true },
            "don't()" => { mul_enabled = false },
            _ => {
                if mul_enabled {
                    sum += capture_and_sum(m.as_str());
                }
            }
        };
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
