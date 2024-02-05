const P1_INPUT_PATH: &'static str = "p1_input.txt";
const P2_INPUT_PATH: &'static str = "p2_input.txt";
const INPUT_PATH: &'static str = "input.txt";

fn main() {
    part_one(INPUT_PATH);
    part_two(INPUT_PATH);
}

fn part_one(input_path: &str) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut total: u32 = 0;

    'for_line: for line in input.lines() {
        let (title, hands) = line.split_once(':').unwrap();
        let (_, id) = title.split_once(' ').unwrap();
        let id = id.parse::<u32>().unwrap();
        let hands = hands.split(';').collect::<Vec<&str>>();

        for hand in hands.iter() {
            let cube_cnts = hand.split(',').collect::<Vec<&str>>();
            for cnt in cube_cnts.iter() {
                let (num, color) = cnt.trim().split_once(' ').unwrap();
                if let Ok(n) = num.parse::<u32>() {
                    match color {
                        "red" => if n > 12 {
                            continue 'for_line;
                        },
                        "green" => if n > 13 {
                            continue 'for_line;
                        },
                        "blue" => if n > 14 {
                            continue 'for_line;
                        },
                        _ => {
                            eprintln!("Bug in the code {}", color);
                        }
                    }
                }
                else {
                    println!("{}", num);
                }
            }
        }

        total += id;
    }

    println!("Total: {}", total);
}

fn part_two(input_path: &str) {
    let input = std::fs::read_to_string(input_path).unwrap();
    let mut total: u32 = 0;

    for line in input.lines() {
        let (_, hands) = line.split_once(':').unwrap();
        let hands = hands.split(';').collect::<Vec<&str>>();

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for hand in hands.iter() {
            let cube_cnts = hand.split(',').collect::<Vec<&str>>();
            for cnt in cube_cnts.iter() {
                let (num, color) = cnt.trim().split_once(' ').unwrap();
                if let Ok(n) = num.parse::<u32>() {
                    match color {
                        "red" => {
                            if n > max_red {
                                max_red = n;
                            }
                        },
                        "green" => {
                            if n > max_green {
                                max_green = n;
                            }
                        },
                        "blue" => {
                            if n > max_blue {
                                max_blue = n;
                            }
                        },
                        _ => {
                            eprintln!("Bug in the code {}", color);
                        }
                    }
                }
                else {
                    println!("{}", num);
                }
            }
        }
        
        total += max_red * max_green * max_blue;
    }

    println!("Total: {}", total);
}