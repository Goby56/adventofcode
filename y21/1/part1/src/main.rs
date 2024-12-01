use std::fs::read_to_string;

fn main() {
    let mut increases = 0;
    let mut previous = Option::None;
    let mut content = read_to_string("./../input.txt").unwrap();
    content.pop();
    for line in content.lines() {
        let measurement = line.parse::<i32>().unwrap();

        if previous != None {
            if measurement > previous.unwrap() {
                increases += 1;
            }
        }
        previous = Some(measurement);
    }
    println!("Increases: {}", increases);
}
