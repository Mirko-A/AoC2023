use std::collections::HashSet;

const P1_INPUT_PATH: &'static str = "p1_input.txt";
const P2_INPUT_PATH: &'static str = "p2_input.txt";
const INPUT_PATH: &'static str = "input.txt";

#[derive(Debug, PartialEq, Eq, Hash)]
struct Symbol {
    value: char,
    loc: (i64, i64),
}

impl Symbol {
    fn new(row: i64, col: i64, value: char) -> Self {
        Symbol {
            value: value,
            loc: (row, col),
        }
    }
}

#[derive(Debug, Default)]
struct PartNumber {
    value: String,
    points: HashSet<(i64, i64)>,
    // points: Vec<(i64, i64)>,
}

impl PartNumber {
    fn get_points_around(row: i64, col: i64) -> HashSet<(i64, i64)> {
        HashSet::<(i64, i64)>::from([
            (row - 1, col - 1), (row, col - 1), (row + 1, col - 1), // left side
            (row - 1, col + 1),   (row, col + 1),   (row + 1, col + 1), // right side
            (row - 1, col), // above
            (row + 1, col), // below
        ])
    }

    fn new(row: i64, col: i64, value: char) -> Self {
        PartNumber {
            value: value.to_string(),
            points: PartNumber::get_points_around(row, col),
        }
    }

    fn add_digit(&mut self, row: i64, col: i64, value: char) {
        self.value += &value.to_string();
        self.points.extend([(row - 1, col + 1), (row, col + 1), (row + 1, col + 1)])
    }
}

#[derive(Debug, Default)]
struct Solution {
    nums: Vec<PartNumber>,
    syms: HashSet<Symbol>,
    answer: i64,
}

fn part_one(input_path: &str) {
    let mut solution = Solution::default();
    let content = std::fs::read_to_string(input_path).unwrap();

    for (row, line) in content.lines().enumerate() {
        let mut current_num: Option<PartNumber> = None;

        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = current_num {
                    num.add_digit(row as i64, col as i64, ch);
                }
                else {
                    current_num = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            }
            else {
                if ch != '.' {
                    solution.syms.insert(Symbol::new(row as i64, col as i64, ch));
                }

                if let Some(num) = current_num.take() {
                    solution.nums.push(num);
                }
            }
        }

        if let Some(num) = current_num {
            solution.nums.push(num);
        }
    }

    for num in &solution.nums {
        let sym_locs = solution.syms.iter().map(|sym| sym.loc).collect();
        if !num.points.is_disjoint(&sym_locs) {
            solution.answer += num.value.parse::<i64>().unwrap();
        }
    }
    
    println!("Part one: {}.", solution.answer);
}

fn part_two(input_path: &str) {
    let mut solution = Solution::default();
    let content = std::fs::read_to_string(input_path).unwrap();

    for (row, line) in content.lines().enumerate() {
        let mut current_num: Option<PartNumber> = None;

        for (col, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                if let Some(ref mut num) = current_num {
                    num.add_digit(row as i64, col as i64, ch);
                } else {
                    current_num = Some(PartNumber::new(row as i64, col as i64, ch));
                }
            } else {
                if ch != '.' {
                    solution.syms.insert(Symbol::new(row as i64, col as i64, ch));
                }

                if let Some(num) = current_num.take() {
                    solution.nums.push(num);
                }
            }
        }

        if let Some(num) = current_num {
            solution.nums.push(num);
        }
    }

    for sym in solution.syms {
        if sym.value != '*' {
            continue;
        }

        let mut neighbor_cnt = 0;
        let mut gear_ratio = 1;
        for num in solution.nums.iter() {
            if num.points.contains(&sym.loc) {
                neighbor_cnt += 1;
                gear_ratio *= num.value.parse::<i64>().expect("Bug in the code.");
            }
        }

        if neighbor_cnt == 2 {
            solution.answer += gear_ratio;
        }
    }
    
    println!("Part two: {}.", solution.answer);
}

fn main() {
    part_one(INPUT_PATH);
    part_two(INPUT_PATH);
}