// const INPUT: &str = include_str!("../input.txt");

// use std::str::Split;

use std::ops::Range;

fn main() {
    part1()
}

fn part1() {
    let input = "71-71,42-72";

    let pair = input.split(",").collect::<Vec<&str>>();

    let nested = pair
        .into_iter()
        .map(|x| x.split("-").collect::<Vec<&str>>());

    // println!("{:?}", nested.collect::<Vec<Vec<&str>>>())

    let ranges: Vec<Range<i32>> = nested
        .map(|pair| Range::<i32> {
            start: pair[0].parse::<i32>().unwrap(),
            end: pair[1].parse::<i32>().unwrap(),
        })
        .collect();

    let g1 = &ranges[0];
    &ranges[1].all(|x| g1.contains(&x));

    println!("{:?}", g1);
    println!("{:?}", ranges);
    // let mapped = split.into_iter().map(|x| {
    //     let y = x.split("-").into_iter().map(|x| x.parse::<i32>());
    //
    //     println!("{:?}", y);
    //     println!("{}", x);
    //     (x, x)
    // });
    // println!("{:?}", mapped.collect::<Vec<(&str, &str)>>())
}
