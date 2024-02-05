use std::collections::HashSet;

const P1_INPUT_PATH: &'static str = "p1_input.txt";
const P2_INPUT_PATH: &'static str = "p2_input.txt";
const INPUT_PATH: &'static str = "input.txt";

#[derive(Default, Debug)]
struct Solution {

}

impl Solution {
    fn parse(input_path: &str) {
        let content = std::fs::read_to_string(input_path).unwrap();
    }

    fn new(input_path: &str) -> Self {
        Self::default()
    }
}

fn part_one(Solution: &Solution) {
    println!("Part one: {}", "unsolved");
}

fn part_two(Solution: &Solution) {
    println!("Part two: {}", "unsolved");
}

fn main() {
    let mut solution = Solution::new(INPUT_PATH);
    part_one(&solution);
    part_two(&solution);
}
