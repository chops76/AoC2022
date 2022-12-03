use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = Vec<String>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn parse_sacks(sack_str: &str) -> (HashSet<char>, HashSet<char>) {
    let left = sack_str[..sack_str.len()/2].chars().collect::<HashSet<char>>();
    let right = sack_str[sack_str.len()/2..].chars().collect::<HashSet<char>>();
    (left,right)
}

fn calc_value(c: &char) -> usize {
    let mut score = *c as u32;
    if score >= 97 {
        score -= 96;
    } else {
        score -= 38;
    }
    score as usize    
}

fn find_score1(l: &HashSet<char>, r: &HashSet<char>) -> usize {
    let common:Vec<_> = l.intersection(&r).collect();
    calc_value(common[0])
}

fn find_score2(sacks: &[HashSet<char>]) -> usize {
    let i1:HashSet<_> = sacks[0].intersection(&sacks[1]).map(|c| *c).collect();
    let i2:Vec<_> = i1.intersection(&sacks[2]).collect();
    calc_value(i2[0])
}

fn part1(sacks: &Input) -> usize {
    sacks.iter().map(|s| parse_sacks(s))
                .map(|(l, r)| find_score1(&l, &r)).sum()
}

fn part2(sacks: &Input) -> usize {
   let parsed: Vec<_> = sacks.iter().map(|s| s.chars().collect::<HashSet<char>>()).collect();
   ((0..parsed.len()).step_by(3)).map(|v| find_score2(&parsed[v..v+3])).sum()
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
