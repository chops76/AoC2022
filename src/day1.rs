use std::io::Read;
use std::fs::File;
use std::time::Instant;

type Input = Vec<Vec<u64>>;

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    fstr.split("\n\n").map(|s| parse_line(s)).collect::<Vec<Vec<u64>>>()
}

fn parse_line(line: &str) -> Vec<u64> {
    line.split("\n").map(|s| s.parse().unwrap()).collect::<Vec<u64>>()
}

fn part1(nums: &Input) -> u64 {
    nums.iter().map(|e| e.iter().sum()).max().unwrap()
}

fn part2(nums: &Input) -> u64 {
    let mut sums = nums.iter().map(|e| e.iter().sum()).collect::<Vec<u64>>();
    sums.sort();
    sums.reverse();
    sums.iter().take(3).sum()
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
        assert_eq!(part1(&nums), 24000);
	}

    #[test]
    fn day1_test2() {
        let nums = parse_input("./input/day1/test.txt");
        assert_eq!(part2(&nums), 45000);
	}
}
