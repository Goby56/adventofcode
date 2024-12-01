use std::fs::read_to_string;

fn main() {
    let mut content = read_to_string("./../input.txt").unwrap();
    content.pop();
    let mut len = 0;
    let mut lines = content.lines();
    let mut occurences: Vec<u32> = as_vec(lines.next().unwrap());

    occurences = lines.fold(occurences, |occurences: Vec<u32>, line: &str| {
        len += 1;
        as_vec(line).iter().zip(occurences.iter()).map(|(a, b)| a + b).collect()
    });

    let mode: Vec<bool> = occurences.iter().map(|&occ| occ > len - occ).collect();
    let gamma = to_decimal(&mode, false);
    let epsilon = to_decimal(&mode, true);

    println!("Product of gamma and epsilon is: {}", gamma * epsilon);
}

fn as_vec(line: &str) -> Vec<u32> {
   return line.trim_end_matches("\n")
       .chars().map(|c| c.to_digit(10).unwrap()).collect();
}

fn to_decimal(bin: &Vec<bool>, flip: bool) -> i32 {
    return bin.iter().rev().enumerate().fold(0, |sum, (i, b)| {
        sum + ((*b ^ flip) as i32) * 2_i32.pow(i as u32)
    });
}
