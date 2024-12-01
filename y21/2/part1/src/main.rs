use std::fs::read_to_string;

fn main() {
    let mut content = read_to_string("./../input.txt").unwrap();
    content.pop();
    let mut pos = [0, 0];
    for lines in content.lines() {
        let instruction = lines.split(" ").collect::<Vec<&str>>();
        let units = instruction[1].parse::<i32>().unwrap();
        match instruction[0] {
            "forward" => {pos[0] += units},
            "up" => {pos[1] -= units},
            "down" => {pos[1] += units},
            _ => panic!("Invalid submarine instruction!")
        }
    }
    println!("Product of pos: {}", pos[0] * pos[1]);
}
