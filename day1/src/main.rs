use std::fs;

fn main() {
    part1();
    part2();
}

fn part1() {
    let contents = fs::read_to_string("input.txt").expect("Should have read the file");

    let max: Option<u32> = contents
        .split("\n\n")
        .map(|x| x.split("\n").flat_map(|x| x.parse::<u32>()).sum())
        .max();

    println!("{}", max.unwrap());
}

fn part2() {
    let contents = fs::read_to_string("input.txt").expect("Should have read the file");

    let mut counts: Vec<u32> = contents
        .split("\n\n")
        .map(|x| x.split("\n").flat_map(|x| x.parse::<u32>()).sum())
        .collect();

    counts.sort();

    println!(
        "{}",
        counts[counts.len() - 1] + counts[counts.len() - 2] + counts[counts.len() - 3]
    );
}
