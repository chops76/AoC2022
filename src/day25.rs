use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<String>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn from_snafu(number: &str) -> i64 {
    number.chars()
          .rev()
          .enumerate()
          .map(|(p,n)| match n { '0'=>0,'1'=>1,'2'=>2,'-'=>-1,'='=>-2,_=>0} * 5_i64.pow(p as u32))
          .sum()
}

fn to_snafu(number: i64) -> String {
    let mut digits = Vec::new();
    let mut remainder = number;
    while remainder != 0 {
        let digit = remainder % 5;
        digits.push(digit);
        remainder /= 5;
    }
    digits.push(0);

    let mut snafu = Vec::new();
    for pos in 0..digits.len()-1 {
        match digits[pos] {
            0 => {
                snafu.push('0');
            },
            1 => {
                snafu.push('1');
            },
            2 => {
                snafu.push('2');
            },
            3 => {
                snafu.push('=');
                digits[pos+1] += 1;
            },
            4 => {
                snafu.push('-');
                digits[pos+1] += 1;
            },
            5 => {
                snafu.push('0');
                digits[pos+1] += 1;
            },
            _ => {
                println!("Invalid digit");
            }
        }
    }
    if snafu[snafu.len() - 1] == '0' {
        snafu.pop();
    }

    snafu.iter().rev().collect()
}

fn part1(numbers: &Vec<String>) -> String {
    to_snafu(numbers.iter().map(|n| from_snafu(n)).sum())
}

fn part2(grid: &Vec<String>) -> usize {
    0
}

pub fn main() {
    let numbers = parse_input("./input/day25/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&numbers);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day23_test1() {
        let inst = parse_input("./input/day25/test.txt");
        assert_eq!(part1(&inst), "2=-1=0");
	}
}