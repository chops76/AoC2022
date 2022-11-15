use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<u32>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| s.parse().unwrap()).collect()
}

fn part1(nums: &Input) -> usize {
    nums.iter().zip(nums.iter().skip(1)).filter(|(a,b)| a < b).count()
}

fn part2(nums: &Input) -> usize {
    nums.iter().zip(nums.iter().skip(3)).filter(|(a,b)| a < b).count()
}

pub fn main() {
    let nums = parse_input("./input/day1/input1.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&nums);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&nums);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        let nums = parse_input("./input/day1/test.txt");
        assert_eq!(part1(&nums), 7);
	}

    #[test]
    fn day1_test2() {
        let nums = parse_input("./input/day1/test.txt");
        assert_eq!(part2(&nums), 5);
	}
}
