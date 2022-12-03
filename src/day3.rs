use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = Vec<String>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn parse_sacks(sack_str: &str) -> Vec<HashSet<char>> {
    let half = sack_str.len()/2;
    [0,half].iter().map(|&v| sack_str[v..v+half].chars().collect::<HashSet<char>>()).collect()
}

fn calc_value(c: &char) -> usize {
    let score = *c as u32;
    (score - if score >= 97 { 96 } else { 38 }) as usize    
}

fn find_score(sacks: &[HashSet<char>]) -> usize {
    let mut i = sacks.iter();
    let mut s:HashSet<char> = i.next().unwrap().clone();
    for sack in i {
        s = s.intersection(&sack).copied().collect();
    }
    calc_value(s.iter().next().unwrap())
}

fn part1(sacks: &Input) -> usize {
    sacks.iter().map(|s| parse_sacks(s))
                .map(|s| find_score(&s)).sum()
}

fn part2(sacks: &Input) -> usize {
   let parsed: Vec<_> = sacks.iter().map(|s| s.chars().collect::<HashSet<char>>()).collect();
   ((0..parsed.len()).step_by(3)).map(|v| find_score(&parsed[v..v+3])).sum()
}

pub fn main() {
    let sacks = parse_input("./input/day3/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&sacks);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&sacks);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        let nums = parse_input("./input/day3/test.txt");
        assert_eq!(part1(&nums), 157);
	}

    #[test]
    fn day1_test2() {
        let nums = parse_input("./input/day3/test.txt");
        assert_eq!(part2(&nums), 70);
	}
}
