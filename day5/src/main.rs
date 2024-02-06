const P1_INPUT_PATH: &'static str = "p1_input.txt";
const P2_INPUT_PATH: &'static str = "p2_input.txt";
const INPUT_PATH: &'static str = "input.txt";

#[derive(Default, Debug)]
struct Map {
    range: std::ops::Range<i64>,
    delta: i64,
}

impl Map {
    fn new(start: i64, end: i64, delta: i64) -> Self {
        Self {
            range: std::ops::Range { start, end },
            delta,
        }
    }
    fn contains(&self, input: i64) -> bool {
        self.range.contains(&input)
    }

    fn apply(&self, input: i64) -> i64 {
        input + self.delta
    }
}

#[derive(Default, Debug)]
struct Mapping {
    maps: Vec<Map>,
}

impl Mapping {
    fn new(maps: Vec<Map>) -> Self {
        Self { maps }
    }

    fn apply(&self, input: i64) -> i64 {
        for map in &self.maps {
            if map.contains(input) {
                return map.apply(input);
            }
        }

        input
    }
}

#[derive(Default, Debug)]
struct Solution {
    seeds1: Vec<i64>,
    seeds2: Vec<std::ops::Range<i64>>,
    mappings: Vec<Mapping>,
}

impl Solution {
    fn read_lines(input_path: &str) -> Vec<String> {
        let content = std::fs::read_to_string(input_path).unwrap();
        let lines = content
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>();
        lines
    }

    fn parse(input_path: &str) -> Solution {
        let mut lines = Self::read_lines(input_path);
        lines.push(String::new());

        let (_, seeds) = lines[0].split_once(": ").unwrap();
        let seeds1 = seeds
            .split_ascii_whitespace()
            .map(|seed| seed.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        let seeds2 = seeds
            .split_ascii_whitespace()
            .map(|seed| seed.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
            .chunks(2)
            .map(|chunk| std::ops::Range {
                start: chunk[0],
                end: chunk[0] + chunk[1],
            })
            .collect::<Vec<_>>();

        let mut opt_maps: Option<Vec<Map>> = None;
        let mut mappings = Vec::new();
        for line in &lines[2..] {
            if line.contains(':') || line.is_empty() {
                if line.is_empty() {
                    let m = Mapping::new(opt_maps.take().unwrap());
                    mappings.push(m);
                }

                continue;
            }
            // Extract mappings
            let nums = line
                .split_ascii_whitespace()
                .map(|num| num.parse::<i64>().unwrap())
                .collect::<Vec<_>>();
            if let Some(ref mut maps) = opt_maps {
                maps.push(Map::new(nums[1], nums[1] + nums[2], nums[0] - nums[1]));
            } else {
                let maps = Vec::from([Map::new(nums[1], nums[1] + nums[2], nums[0] - nums[1])]);
                opt_maps = Some(maps);
            }
        }

        Solution {
            seeds1,
            seeds2,
            mappings,
        }
    }

    fn new(input_path: &str) -> Self {
        Self::parse(input_path)
    }
}

fn part_one(solution: &Solution) {
    let mut min = i64::MAX;
    for &seed in &solution.seeds1 {
        let mut result = seed;

        for m in &solution.mappings {
            result = m.apply(result);
        }

        min = std::cmp::min(min, result);
    }
    println!("Part one: {}", min);
}

fn part_two(solution: &Solution) {
    let mut min = i64::MAX;
    for seed_range in &solution.seeds2 {
        for seed in seed_range.clone() {
            let mut result = seed;

            for m in &solution.mappings {
                result = m.apply(result);
            }

            min = std::cmp::min(min, result);
        }
    }
    println!("Part two: {}", min);
}

fn main() {
    let solution = Solution::new(INPUT_PATH);
    part_one(&solution);
    part_two(&solution);
}
