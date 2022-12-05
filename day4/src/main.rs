// const INPUT: &str = include_str!("../input.txt");

use std::str::Split;

fn main() {
    part1()
}

fn part1() {
    let input = "71-71,42-72";

    let split: Split<&str> = input.split(",");
    let mapped = split.into_iter().map(|x| {
        let y = x.split("-").map(|x| x.parse::<i32>());

        println!("{:?}", y);
        println!("{}", x);
        (x, x)
    });
    println!("{:?}", mapped.collect::<Vec<(&str, &str)>>())
}
