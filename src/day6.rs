use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = String;

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    
    fstr
}

fn find_distinct(code: &str, num_chars: usize) -> usize {
    for i in num_chars..=code.len() {
        if code[i-num_chars..i].chars().collect::<HashSet<char>>().len() == num_chars {
            return i;
        }
    }
    0    
}

fn part1(code: &str) -> usize {
    find_distinct(code, 4)
}

fn part2(code: &str) -> usize {
    find_distinct(code, 14)
}

pub fn main() {
    let code = parse_input("./input/day6/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&code);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&code);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day6_test1() {
        let code = parse_input("./input/day6/test.txt");
        assert_eq!(part1(&code), 7);
	}

    #[test]
    fn day6_test2() {
        let code = parse_input("./input/day6/test.txt");
        assert_eq!(part2(&code), 19);
	}

    #[test]
    fn day6_test3() {
        let code = parse_input("./input/day6/test2.txt");
        assert_eq!(part1(&code), 5);
	}

    #[test]
    fn day6_test4() {
        let code = parse_input("./input/day6/test2.txt");
        assert_eq!(part2(&code), 23);
	}
}
