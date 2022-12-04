use std::fs;

fn main() {
    part1() 
}

fn part1() {
    let contents = fs::read_to_string("input.txt").expect("Should have read the file");

    let max = contents
        .split("\n\n")
        .map(|x| {
            x.split("\n").flat_map(|x| x.parse::<u32>()).sum::<u32>()
        })
        .max();

    println!("{:?}", max);
}
