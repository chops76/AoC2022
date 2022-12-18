use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;
use std::collections::VecDeque;

type Input = HashSet<(i64,i64,i64)>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> (i64,i64,i64) {
    let spl = line.split(",").collect::<Vec<&str>>();

    (spl[0].parse().unwrap(), spl[1].parse().unwrap(), spl[2].parse().unwrap())
}

fn neighbors((x,y,z): &(i64,i64,i64)) -> HashSet<(i64,i64,i64)> {
    let mut ret_set = HashSet::new();
    ret_set.insert((*x - 1,*y,*z));
    ret_set.insert((*x + 1,*y,*z));
    ret_set.insert((*x,*y - 1,*z));
    ret_set.insert((*x,*y + 1,*z));
    ret_set.insert((*x,*y,*z - 1));
    ret_set.insert((*x,*y,*z + 1));
    ret_set
}

fn part1(blocks: &Input) -> usize {
    blocks.iter()
        .map(|b| 6 - blocks.intersection(&neighbors(b)).collect::<Vec<&(i64,i64,i64)>>().len())
        .sum()
}

fn part2(blocks: &Input) -> usize {
    let mut air = HashSet::new();
    let mut to_check = VecDeque::new();
    to_check.push_back((-1,-1,-1));
    air.insert((-1,-1,-1));
    while let Some(block) = to_check.pop_front() {
        let n = neighbors(&block);
        for (x,y,z) in n {
            if x >= -1 && x <= 26 && y >= -1 && y <= 26 && z >= -1 && z <= 26 
                      && !air.contains(&(x,y,z)) && !blocks.contains(&(x,y,z)) {
                to_check.push_back((x,y,z));
                air.insert((x,y,z));
            }
        }
    }
    blocks.iter()
        .map(|b| air.intersection(&neighbors(b)).collect::<Vec<&(i64,i64,i64)>>().len())
        .sum()
}

pub fn main() {
    let inst = parse_input("./input/day18/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&inst);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&inst);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day18_test1() {
        let inst = parse_input("./input/day18/test.txt");
        assert_eq!(part1(&inst), 64);
	}

    #[test]
    fn day18_test2() {
        let inst = parse_input("./input/day18/test.txt");
        assert_eq!(part2(&inst), 58);
	}
}
