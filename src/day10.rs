use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<(String, i64)>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> (String, i64) {
    let spl = line.split_ascii_whitespace().collect::<Vec<&str>>();
    (spl[0].to_string(), if spl[0] == "noop" {0} else {spl[1].parse().unwrap()})
}

fn part1(inst: &Input) -> i64 {
    let mut sum = 0;
    let mut x:i64 = 1;
    let mut cycle = 1;
    let mut ip = 0;
    while cycle <= 220 {
        if inst[ip].0 == "noop" {
            if (cycle - 20) % 40 == 0 {
                sum += ((cycle + 1) / 10 * 10) * x;
            }
            cycle += 1;            
        } else {
            if (cycle - 20) % 40 == 0 || (cycle - 20) % 40 == 39 || cycle == 19 {
                sum += ((cycle + 1) / 10 * 10) * x;
            }
            cycle += 2;
            x += inst[ip].1;
        }
        ip += 1;
    }
    sum
}

fn disp_pixel(x: i64, cycle: i64) {
    if (x - (cycle % 40)).abs() <= 1 {
        print!("#");
    } else {
        print!(".");
    }
}

fn part2(inst: &Input) {
    let mut x:i64 = 1;
    let mut cycle = 0;
    let mut ip = 0;
    while cycle < 240 {
        if inst[ip].0 == "noop" {
            disp_pixel(x, cycle);
            cycle += 1;
            if cycle % 40 == 0 {
                println!();
            }           
        } else {
            for _ in 0..2 {
                disp_pixel(x, cycle);
                cycle += 1;
                if cycle % 40 == 0 {
                    println!();
                } 
            }
            x += inst[ip].1;
        }
        ip += 1;
    }
}

pub fn main() {
    let inst = parse_input("./input/day10/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&inst);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    part2(&inst);
    let p2_time = p2_timer.elapsed();
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day8_test1() {
        let inst = parse_input("./input/day10/test.txt");
        assert_eq!(part1(&inst), 13140);
	}
}
