const INPUT: &str = include_str!("../input.txt");

fn main() {
    part1();
    part2();
}

// A -> rock (1)
// B -> paper (2)
// C -> scissors (3)
fn part1() {
    let mut total = 0;
    for line in INPUT.lines() {
        total = total
            + match line {
                "A X" => 4,
                "A Y" => 8,
                "A Z" => 3,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 7,
                "C Y" => 2,
                "C Z" => 6,
                _ => 0,
            };
    }
    println!("{}", total);
}

// A -> rock (1)
// B -> paper (2)
// C -> scissors (3)
//
// X -> lose
// Y -> tie
// Z -> win
fn part2() {
    let mut total = 0;
    for line in INPUT.lines() {
        total = total
            + match line {
                "A X" => 3,
                "A Y" => 4,
                "A Z" => 8,
                "B X" => 1,
                "B Y" => 5,
                "B Z" => 9,
                "C X" => 2,
                "C Y" => 6,
                "C Z" => 7,
                _ => 0,
            };
    }
    println!("{}", total);
}
