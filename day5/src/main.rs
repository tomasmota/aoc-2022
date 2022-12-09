#![feature(iter_array_chunks)]
const INPUT: &str = include_str!("../input.txt");

use std::collections::HashMap;

fn main() {
    part1();
}

fn part1() {
    // Starting in line with index 7, parse crates and push them into respective stack. Split on spaces
    // Do this until the first line is reached
    // Starting on line with index 10, pop the given number of crates, popping and pushing them one at
    // a time.

    // Create a map, mapping the index to each of the 9 stacks
    let mut deck: HashMap<usize, Vec<char>> = HashMap::new();
    for i in 1..=9 {
        deck.insert(i, Vec::new());
    }

    // Get starting setup lines
    let start_lines: Vec<&str> = INPUT
        .lines()
        .take_while(|line| line.trim_start().starts_with(|c| c == '['))
        .collect();

    // Put these lines into the hashmap
    for line in start_lines.into_iter().rev() {
        for i in (1..34).step_by(4) {
            let letter = line.chars().nth(i).expect("exists");
            let position = (i + 3) / 4;
            deck.get_mut(&position).unwrap().push(letter)
        }
    }

    // Iterate over movements
}
