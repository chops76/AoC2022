use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<(char,char)>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten()
        .map(|s| (s.chars().nth(0).unwrap(),s.chars().nth(2).unwrap())).collect()
}

fn score(round: (char,char)) -> usize {
    match round.1 {
        'X' => 1 + match round.0 { 'A' => 3, 'B' => 0, _ => 6 },
        'Y' => 2 + match round.0 { 'A' => 6, 'B' => 3, _ => 0 },
        'Z' => 3 + match round.0 { 'A' => 0, 'B' => 6, _ => 3 },
        _ => { println!("Bad Data"); 0 }
    }
}

fn score2(round: (char,char)) -> usize {
    match round.1 {
        'X' => match round.0 { 'A' => 3, 'B' => 1, _ => 2 },
        'Y' => 3 + match round.0 { 'A' => 1, 'B' => 2, _ => 3 },
        'Z' => 6 + match round.0 { 'A' => 2, 'B' => 3, _ => 1 },
        _ => { println!("Bad Data"); 0 }
    }
}

fn part1(guide: &Input) -> usize {
    guide.iter().map(|s| score(*s)).sum()
}

fn part2(guide: &Input) -> usize {
   guide.iter().map(|s| score2(*s)).sum()
}

pub fn main() {
    let guide = parse_input("./input/day2/input.txt");

    let p1_timer = Instant::now();
    let p1_result = part1(&guide);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&guide);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        let nums = parse_input("./input/day2/test.txt");
        assert_eq!(part1(&nums), 15);
	}

    #[test]
    fn day1_test2() {
        let nums = parse_input("./input/day2/test.txt");
        assert_eq!(part2(&nums), 12);
	}
}
