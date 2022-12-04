use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<((usize,usize),(usize,usize))>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> ((usize,usize),(usize,usize)) {
    let spl:Vec<&str> = line.split(',').collect();

    (parse_range(spl[0]),parse_range(spl[1]))
}

fn parse_range(srange: &str) -> (usize,usize) {
    let spl:Vec<&str> = srange.split('-').collect();
    (spl[0].parse().unwrap(), spl[1].parse().unwrap())
}

fn contained(l: &(usize, usize), r: &(usize, usize)) -> bool {
    (l.0 >= r.0 && l.1 <= r.1) || (r.0 >= l.0 && r.1 <= l.1)
}

fn overlap(l: &(usize,usize), r: &(usize,usize)) -> bool {
    (l.0 >= r.0 && l.0 <= r.1) || (l.1 >= r.0 && l.1 <= r.1) ||
    (r.0 >= l.0 && r.0 <= l.1) || (r.1 >= l.0 && r.1 <= l.1)
}

fn part1(ranges: &Input) -> usize {
    ranges.iter().filter(|(l,r)| contained(l, r)).count()
}

fn part2(ranges: &Input) -> usize {
    ranges.iter().filter(|(l,r)| overlap(l, r)).count()
}

pub fn main() {
    let ranges = parse_input("./input/day4/input.txt");
    println!("{:?}", ranges);
    let p1_timer = Instant::now();
    let p1_result = part1(&ranges);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&ranges);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        let nums = parse_input("./input/day4/test.txt");
        assert_eq!(part1(&nums), 2);
	}

    #[test]
    fn day1_test2() {
        let nums = parse_input("./input/day4/test.txt");
        assert_eq!(part2(&nums), 4);
	}
}
