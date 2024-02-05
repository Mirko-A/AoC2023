const P1_INPUT_PATH: &'static str = "p1_input.txt";
const P2_INPUT_PATH: &'static str = "p2_input.txt";
const INPUT_PATH: &'static str = "input.txt";

fn main() {
    part_one(INPUT_PATH);
    part_two(INPUT_PATH);
}

fn part_one(input_path: &str) {
    let content = std::fs::read_to_string(input_path)
        .expect(format!("File {} not found.", input_path)
        .as_str());

    let mut total = 0;

    for line in content.lines() {
        let nums = line.chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch as u8 - '0' as u8)
            .collect::<Vec<u8>>();

        let first = nums.first().unwrap();
        let last = nums.last().unwrap();
        total += (*first as u32 * 10) + *last as u32;
    }

    println!("{}", total)
}

fn part_two(input_path: &str) {
    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let content = std::fs::read_to_string(input_path)
        .expect(format!("File {} not found.", input_path)
        .as_str());

    let mut total = 0u32;
    
    for line in content.lines() {
        let mut first: u8 = 0;
        let mut last: u8 = 0;

        'line: for (i, ch) in line.chars().enumerate() {
            if ch.is_ascii_digit() {
                first = ch as u8 - '0' as u8;
                break;
            }
            for n in nums {
                if let Some(n_ch) = n.chars().nth(0) {
                    if (n_ch != ch) || ((i + n.len()) > line.len()) { continue; }

                    let maybe_num = &line[i..(i + n.len())]; 
                    if maybe_num == n {
                        first = match maybe_num {
                            "one"   => 1,
                            "two"   => 2,
                            "three" => 3,
                            "four"  => 4,
                            "five"  => 5,
                            "six"   => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine"  => 9,
                            _ => panic!("Bug in the code on line: {}", line!())
                        };
                        break 'line;
                    }
                }
            }
        }

        'line_rev: for (i, ch) in line.chars().rev().enumerate() {
            let i_rev = line.len() - i - 1;
            if ch.is_ascii_digit() {
                last = ch as u8 - '0' as u8;
                break 'line_rev;
            }
            for n in nums {
                if let Some(n_ch) = n.chars().nth(0) {
                    if (n_ch != ch) || ((i_rev + n.len()) > line.len()) { continue; }

                    let maybe_num = &line[i_rev..(i_rev + n.len())]; 
                    if maybe_num == n {
                        last = match maybe_num {
                            "one"   => 1,
                            "two"   => 2,
                            "three" => 3,
                            "four"  => 4,
                            "five"  => 5,
                            "six"   => 6,
                            "seven" => 7,
                            "eight" => 8,
                            "nine"  => 9,
                            _ => panic!("Bug in the code on line: {}", line!())
                        };
                        break 'line_rev;
                    }
                }
            }
        }

        total += first as u32 * 10 + last as u32;
    }

    println!("{}", total)
}