use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;

type Input = Vec<Monkey>;

#[derive(Debug)]
struct Monkey {
    starting: Vec<i64>,
    to_mul: i64,
    to_add: i64,
    div_test: i64,
    pass_throw: usize,
    fail_throw: usize
}

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    fstr.split("\n\n").map(|s| parse_monkey(s)).collect()
}

fn parse_monkey(monkey: &str) -> Monkey {
    let spl = monkey.split("\n").collect::<Vec<&str>>();
    let num_list = spl[1].split(": ").collect::<Vec<&str>>();
    let starting_vec = num_list[1].split(", ").map(|s| s.parse().unwrap()).collect();
    let op_list = spl[2].split_whitespace().collect::<Vec<&str>>();
    let to_mul;
    let to_add;
    if op_list[4] == "*" {
        if op_list[5] == "old" {
            to_mul = -1;
        } else {
            to_mul = op_list[5].parse().unwrap();
        }
        to_add = 0;
    } else {
        to_mul = 1;
        to_add = op_list[5].parse().unwrap();
    }
    let test_spl = spl[3].split_whitespace().collect::<Vec<&str>>();
    let pass_spl = spl[4].split_whitespace().collect::<Vec<&str>>();
    let fail_spl = spl[5].split_whitespace().collect::<Vec<&str>>();
    Monkey { starting: starting_vec, to_mul: to_mul, 
            to_add: to_add, div_test: test_spl[3].parse().unwrap(), 
            pass_throw: pass_spl[5].parse().unwrap(), fail_throw: fail_spl[5].parse().unwrap() }
}

fn part1(monkeys: &Input) -> usize {
    let mut ml = Vec::new();
    for i in 0..monkeys.len() {
        ml.push(monkeys[i].starting.iter().map(|v| *v).collect::<VecDeque<i64>>());
    }
    let mut inspections = vec![0;monkeys.len()];
    for _ in 0..20 {
        for turn in 0..ml.len() {
            while let Some(cur_val) = ml[turn].pop_front() {
                inspections[turn] += 1;
                let mut new_val = cur_val;
                new_val += monkeys[turn].to_add;
                if monkeys[turn].to_mul == -1 {
                    new_val *= new_val;
                } else {
                    new_val *= monkeys[turn].to_mul;
                }
                new_val /= 3;
                if new_val % monkeys[turn].div_test == 0 {
                    ml[monkeys[turn].pass_throw].push_back(new_val);
                } else {
                    ml[monkeys[turn].fail_throw].push_back(new_val);
                }
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn part2(monkeys: &Input) -> usize {
    let mut ml = Vec::new();
    for i in 0..monkeys.len() {
        ml.push(monkeys[i].starting.iter().map(|v| *v).collect::<VecDeque<i64>>());
    }
    let mod_num: i64 = monkeys.iter().map(|m| m.div_test).product();
    let mut inspections = vec![0;monkeys.len()];
    for _ in 0..10000 {
        for turn in 0..ml.len() {
            while let Some(cur_val) = ml[turn].pop_front() {
                inspections[turn] += 1;
                let mut new_val = cur_val;
                new_val += monkeys[turn].to_add;
                if monkeys[turn].to_mul == -1 {
                    new_val *= new_val;
                } else {
                    new_val *= monkeys[turn].to_mul;
                }
                new_val %= mod_num;
                if new_val % monkeys[turn].div_test == 0 {
                    ml[monkeys[turn].pass_throw].push_back(new_val);
                } else {
                    ml[monkeys[turn].fail_throw].push_back(new_val);
                }
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

pub fn main() {
    let inst = parse_input("./input/day11/input.txt");
    
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
    fn day11_test1() {
        let inst = parse_input("./input/day11/test.txt");
        assert_eq!(part1(&inst), 10605);
	}

    #[test]
    fn day11_test2() {
        let inst = parse_input("./input/day11/test.txt");
        assert_eq!(part2(&inst), 2713310158);
	}
}
