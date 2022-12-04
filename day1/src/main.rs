use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have read the file");
    let split:Vec<&str> = contents.split("\n\n").collect();
    let mut max = 0;
    for elf in split {
        let sum:i32 = elf
            .split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.parse::<i32>().unwrap())
            .sum();
        if sum > max {
            max = sum
        }
    }

    println!("{max}");
}
