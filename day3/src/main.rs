const INPUT: &str = include_str!("../input.txt");
const LETTERS: &str = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    part1()
}

fn part1() {
    let lines = INPUT.lines();

    let mut sum = 0;
    for line in lines {
        let left = line.get(..line.len() / 2).unwrap();
        let right = line.get(line.len() / 2..).unwrap();

        sum = sum + LETTERS.find(|x| x == common_char(left, right)).unwrap();
    }
}

fn common_char(str1: &str, str2: &str) -> char {
    for lc in str1.chars() {
        for rc in str2.chars() {
            if lc == rc {
                return lc;
            }
        }
    }
    panic!("unknown char");
}
