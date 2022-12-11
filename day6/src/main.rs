use itertools::Itertools;
use std::collections::VecDeque;

const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
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

fn part2() {
    let chars = INPUT
        .lines()
        .nth(0)
        .expect("first line is present")
        .char_indices();

    let mut window: VecDeque<char> = VecDeque::new();

    for (i, c) in chars {
        // for first 14 chars, just put them in the window
        if i < 14 {
            window.push_back(c);
        } else {
            // after filling the window, move through until all chars are unique
            if window.clone().into_iter().all_unique() {
                println!("{}", i);
                return;
            }
            window.pop_front();
            window.push_back(c);
        }
    }
}
