advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut ids = line.split_whitespace();
        list1.push(ids.next().unwrap().parse::<i32>().unwrap());
        list2.push(ids.next().unwrap().parse::<i32>().unwrap());
    }
    list1.sort();
    list2.sort();

    Some(list1.iter().zip(list2.iter()).fold(0, |dist, (&x, &y)| dist + (x - y).abs()) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1 = vec![];
    let mut list2 = vec![];
    for line in input.lines() {
        let mut ids = line.split_whitespace();
        list1.push(ids.next().unwrap().parse::<i32>().unwrap());
        list2.push(ids.next().unwrap().parse::<i32>().unwrap());
    }
    let mut score = 0;
    for id1 in list1.iter() {
        let mut count = 0;
        for id2 in list2.iter() {
            if id1 == id2 {
                count += 1;
            }
        }
        score += id1 * count;
    }
    Some(score as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
