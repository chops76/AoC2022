use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<Vec<(usize,usize)>>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> Vec<(usize,usize)> {
    let spl = line.split(" -> ").collect::<Vec<&str>>();

    spl.iter().map(|s| parse_point(s)).collect()
}

fn parse_point(line: &str) -> (usize,usize) {
    let spl = line.split(",").collect::<Vec<&str>>();
    (spl[0].parse().unwrap(), spl[1].parse().unwrap())
}

fn calc(points: &Input, part2: bool) -> usize {
    let mut grid = vec![vec!['.';1000];500];
    let mut lowest = 0;
    for l in points {
        for i in 0..l.len() - 1 {
            if l[i].0 == l[i+1].0 {
                for count in std::cmp::min(l[i].1,l[i+1].1)..=std::cmp::max(l[i].1,l[i+1].1) {
                    grid[count][l[i].0] = '#';
                    if count > lowest {
                        lowest = count;
                    }
                }
            } else {
                for count in std::cmp::min(l[i].0,l[i+1].0)..=std::cmp::max(l[i].0,l[i+1].0) {
                    grid[l[i].1][count] = '#';
                    if l[1].1 > lowest {
                        lowest = l[1].1;
                    }
                }
            }
        }
    }
    let mut sand_count = 0;
    loop {
        let mut x = 500;
        let mut y = 0;
        loop {
            if y > lowest {
                if part2 {
                    break;
                }
                return sand_count;
            }
            if grid[y+1][x] == '.' {
                y += 1;
            } else if grid[y+1][x-1] == '.' {
                y += 1;
                x -= 1;
            } else if grid[y+1][x+1] == '.' {
                y += 1;
                x += 1;
            } else {
                break;
            }
        }
        grid[y][x] = '+';
        sand_count += 1;
        if part2 && x == 500 && y == 0 {
            return sand_count;
        }
    }
}

pub fn main() {
    let inst = parse_input("./input/day14/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = calc(&inst, false);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = calc(&inst, true);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day14_test1() {
        let inst = parse_input("./input/day14/test.txt");
        assert_eq!(calc(&inst, false), 24);
	}

    #[test]
    fn day14_test2() {
        let inst = parse_input("./input/day14/test.txt");
        assert_eq!(calc(&inst, true), 93);
	}
}
