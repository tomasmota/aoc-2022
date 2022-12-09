#![feature(iter_array_chunks)]
const INPUT: &str = include_str!("../input.txt");

use std::collections::HashMap;

fn main() {
    part1();
    part2();
}

fn part1() {
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
            let letter = line.chars().nth(i).unwrap();
            if !letter.is_whitespace() {
                let position = (i + 3) / 4;
                deck.get_mut(&position).unwrap().push(letter)
            }
        }
    }

    // Iterate over movements
    for line in INPUT.lines().skip(10) {
        let mut sections = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok());
        let amount = sections.next().unwrap();
        let from_nr = sections.next().unwrap();
        let to_nr = sections.next().unwrap();

        for _ in 0..amount {
            let from = deck.get_mut(&(from_nr as usize)).unwrap();
            let letter = from.pop();
            let to = deck.get_mut(&(to_nr as usize)).unwrap();
            to.push(letter.expect("we have the letter"));
        }
    }

    // Get top crates
    for i in 1..=9 {
        let top_crate = deck
            .get(&i)
            .expect("stack is there")
            .last()
            .expect("letter is there");
        print!("{}", top_crate);
    }
    println!()
}

fn part2() {
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
            let letter = line.chars().nth(i).unwrap();
            if !letter.is_whitespace() {
                let position = (i + 3) / 4;
                deck.get_mut(&position).unwrap().push(letter)
            }
        }
    }

    // Iterate over movements
    for line in INPUT.lines().skip(10) {
        let mut sections = line
            .split_whitespace()
            .filter_map(|x| x.parse::<u32>().ok());
        let amount = sections.next().unwrap();
        let from_nr = sections.next().unwrap();
        let to_nr = sections.next().unwrap();

        let mut temp_crates: Vec<char> = Vec::new();
        for _ in 0..amount {
            let from = deck.get_mut(&(from_nr as usize)).unwrap();
            let letter = from.pop();
            temp_crates.push(letter.expect("we have the letter"));
        }

        for letter in temp_crates.into_iter().rev() {
            let to = deck.get_mut(&(to_nr as usize)).unwrap();
            to.push(letter);
        }
    }

    // Get top crates
    for i in 1..=9 {
        let top_crate = deck
            .get(&i)
            .expect("stack is there")
            .last()
            .expect("letter is there");
        print!("{}", top_crate);
    }
    println!()
}
