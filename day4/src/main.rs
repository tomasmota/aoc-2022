const INPUT: &str = include_str!("../input.txt");

use std::ops::Range;

fn main() {
    part1()
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

fn contained(left: &Range<i32>, right: &Range<i32>) -> bool {
    return (left.start <= right.start && left.end >= right.end)
        || (left.start >= right.start && left.end <= right.end);
}
