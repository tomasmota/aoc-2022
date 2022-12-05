#![feature(iter_array_chunks)]
const INPUT: &str = include_str!("../input.txt");
const LETTERS: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    part1();
    part2();
}

fn part1() {
    let lines = INPUT.lines();

    let mut sum = 0;
    for line in lines {
        let left = line.get(..line.len() / 2).unwrap();
        let right = line.get(line.len() / 2..).unwrap();

        sum += LETTERS.find(|x| x == common_char_in2(left, right)).unwrap();
    }
    println!("{}", sum);
}

fn part2() {
    let lines = INPUT.lines();

    let mut sum = 0;
    for [x, y, z] in lines.into_iter().array_chunks() {
        sum += LETTERS.find(|s| s == common_char_in3(x, y, z)).unwrap();
    }
    println!("{}", sum);
}

fn common_char_in3(str1: &str, str2: &str, str3: &str) -> char {
    for l1 in str1.chars() {
        for l2 in str2.chars() {
            if l1 == l2 {
                for l3 in str3.chars() {
                    if l2 == l3 {
                        return l1;
                    }
                }
            }
        }
    }
    panic!("unknown char");
}

fn common_char_in2(str1: &str, str2: &str) -> char {
    for lc in str1.chars() {
        for rc in str2.chars() {
            if lc == rc {
                return lc;
            }
        }
    }
    panic!("unknown char");
}
