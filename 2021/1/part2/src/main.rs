use std::fs::read_to_string;


/*
0  199  A
1  200  a B    
2  208  a b C  
3  210  D b c
4  200  d E c
5  207  d e F
6  240  G e f  
7  269  g H f
8  260  g h I
9  263    h i
10 270      i
*/

fn main() {
    let mut increases = 0;
    let mut previous = i32::MAX;
    let mut content = read_to_string("./../input.txt").unwrap();
    content.pop();
    let mut sums: [i32; 3] = [0, 0, 0];
    for (i, line) in content.lines().enumerate() {
        if i >= 2 {
            if sums[i % 3] > previous {
                increases += 1;
            }
            previous = sums[i % 3];
        }
        sums[i % 3] = 0;
        let measurement = line.parse::<i32>().unwrap();
        for j in 0..3 {
            sums[j] += measurement;
        }
    }
    println!("Increases: {}", increases);
}
