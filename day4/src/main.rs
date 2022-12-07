const INPUT: &str = include_str!("../input.txt");

use std::{cmp, ops::Range};

fn main() {
    part1();
    part2();
}

fn part1() {
    let sum = INPUT.lines().fold(0, |accum, x| {
        let n = x
            .split(",")
            .map(|x| {
                let mut split = x.split("-").map(|x| x.parse::<i32>().unwrap());
                return Range {
                    start: split.next().unwrap(),
                    end: split.next().unwrap(),
                };
            })
            .collect::<Vec<Range<i32>>>();

        let left = n.get(0).unwrap();
        let right = n.get(1).unwrap();
        if contained(left, right) {
            return accum + 1;
        }

        return accum;
    });

    println!("{}", sum);
}

fn part2() {
    let sum = INPUT.lines().fold(0, |accum, x| {
        let n = x
            .split(",")
            .map(|x| {
                let mut split = x.split("-").map(|x| x.parse::<i32>().unwrap());
                return Range {
                    start: split.next().unwrap(),
                    end: split.next().unwrap(),
                };
            })
            .collect::<Vec<Range<i32>>>();

        let left = n.get(0).unwrap();
        let right = n.get(1).unwrap();
        if overlapping(left, right) {
            return accum + 1;
        }

        return accum;
    });

    println!("{}", sum);
}

fn overlapping(left: &Range<i32>, right: &Range<i32>) -> bool {
    let total_range = cmp::max(left.end, right.end) - cmp::min(left.start, right.start);
    let summed_ranges = (left.end - left.start) + (right.end - right.start);
    return summed_ranges >= total_range;
}

fn contained(left: &Range<i32>, right: &Range<i32>) -> bool {
    return (left.start <= right.start && left.end >= right.end)
        || (left.start >= right.start && left.end <= right.end);
}
