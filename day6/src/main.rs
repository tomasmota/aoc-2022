#![feature(generic_arg_infer)]
#![feature(iter_array_chunks)]
use itertools::Itertools;

const INPUT: &str = include_str!("../input.test.txt");

fn main() {
    part1()
}

fn part1() {
    let stream = INPUT.lines().nth(0).expect("first line is present");

    let mut index = 0;
    for (a, b, c, d) in stream.chars().tuple_windows() {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            break;
        }
        index += 1;
    }

    println!("{}", index + 4);
}
