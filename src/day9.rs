use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = Vec<(char, usize)>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> (char, usize) {
    let spl = line.split_ascii_whitespace().collect::<Vec<&str>>();
    (spl[0].chars().next().unwrap(), spl[1].parse().unwrap())
}

fn calc_tail(hx: i32, hy: i32, tx: &mut i32, ty: &mut i32) {
    if hx - *tx == 2 {
        *tx += 1;
        if hy > *ty {
            *ty += 1;
        } else if hy < *ty {
            *ty -= 1;
        }
    } else if *tx - hx == 2 {
        *tx -= 1;
        if hy > *ty {
            *ty += 1;
        } else if hy < *ty {
            *ty -= 1;
        }
    } else if hy - *ty == 2 {
        *ty += 1;
        if hx > *tx {
            *tx += 1;
        } else if hx < *tx {
            *tx -= 1;
        }
    } else if *ty - hy == 2 {
        *ty -= 1;
        if hx > *tx {
            *tx += 1;
        } else if hx < *tx {
            *tx -= 1;
        }
    }
}

fn part1(moves: &Input) -> usize {
    let mut seen = HashSet::new();
    let mut hx:i32 = 0;
    let mut hy:i32 = 0;
    let mut tx:i32 = 0;
    let mut ty:i32 = 0;
    seen.insert((tx,ty));
    for (dir,count) in moves {
        for _ in 0..*count {
            match *dir {
                'R' => hx += 1,
                'L' => hx -= 1,
                'U' => hy += 1,
                'D' => hy -= 1,
                _ => {
                    println!("Illegal Instruction")
                }
            }
            calc_tail(hx, hy, &mut tx, &mut ty);
            seen.insert((tx,ty));
        }
    }
    seen.len()
}

fn part2(moves: &Input) -> usize {
    let mut seen = HashSet::new();
    let mut knots = vec![(0,0);10];

    seen.insert((0,0));
    for (dir,count) in moves {
        for _ in 0..*count {
            match *dir {
                'R' => knots[0].0 += 1,
                'L' => knots[0].0 -= 1,
                'U' => knots[0].1 += 1,
                'D' => knots[0].1 -= 1,
                _ => {
                    println!("Illegal Instruction")
                }
            }
            for i in 1..10 {
                let mut tx = knots[i].0;
                let mut ty = knots[i].1;
                calc_tail(knots[i-1].0, knots[i-1].1, &mut tx, &mut ty);
                knots[i] = (tx, ty);
            }
            seen.insert((knots[9].0, knots[9].1));
        }
    }
    seen.len()
}

pub fn main() {
    let moves = parse_input("./input/day9/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&moves);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&moves);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day8_test1() {
        let moves = parse_input("./input/day9/test.txt");
        assert_eq!(part1(&moves), 13);
	}

    #[test]
    fn day8_test2() {
        let moves = parse_input("./input/day9/test.txt");
        assert_eq!(part2(&moves), 1);
	}
}
