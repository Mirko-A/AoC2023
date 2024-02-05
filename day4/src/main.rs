use std::collections::HashSet;

const P1_INPUT_PATH: &'static str = "p1_input.txt";
const P2_INPUT_PATH: &'static str = "p2_input.txt";
const INPUT_PATH: &'static str = "input.txt";

fn part_one(input_path: &str) {
    let content = std::fs::read_to_string(input_path).unwrap();
    let mut  total = 0;
    
    for line in content.lines() {
        let all_nums = line.split_once(": ").unwrap().1;
        let (winning_nums, my_nums) = all_nums.split_once(" | ").unwrap();
        let winning_nums = winning_nums
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let my_nums = my_nums
            .split_whitespace()
            .map(|n| n.parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let matches = my_nums.intersection(&winning_nums).collect::<HashSet<_>>();
        if matches.is_empty() {
            continue;
        }

        let mut  points = 1;
        for _ in 1..matches.len() {
            points *= 2;
        }

        total += points;
    }
    println!("{}", total);
}

fn part_two(input_path: &str) {
    println!("Part two: {}.", "unsolved");
}

fn main() {
    part_one(INPUT_PATH);
    part_two(P2_INPUT_PATH);
}
