use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(Clone)]
enum Monkey {
    Number(i64),
    Operation(String,String,String)
}

type Input = HashMap<String, Monkey>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    let mut hm = HashMap::new();
    for l in BufReader::new(f).lines().flatten() {
        let spl = l.split_whitespace().collect::<Vec<&str>>();
        if spl.len() == 2 {
            hm.insert(spl[0][0..spl[0].len()-1].to_string(), Monkey::Number(spl[1].parse().unwrap()));
        } else {
            hm.insert(spl[0][0..spl[0].len()-1].to_string(), Monkey::Operation(spl[1].to_string(),
                      spl[2].to_string(), spl[3].to_string()));
        }
    }
    hm
}

fn part1(m: &Input) -> i64 {
    let mut monkeys = m.clone();
    loop {
        for (key,_) in m {
            if let Monkey::Operation(s1,op,s2) = &monkeys[key] {
                if let Monkey::Number(v1) = monkeys[s1] {
                    if let Monkey::Number(v2) = monkeys[s2] {
                        let new_val = match op.as_str() {
                            "+" => {v1 + v2},
                            "-" => {v1 - v2},
                            "*" => {v1 * v2},
                            _ => {v1 / v2}
                        };
                        if key == "root" {
                            return new_val;
                        }
                        monkeys.insert(key.to_string(), Monkey::Number(new_val));
                    }
                }
            }
        }
    }
}

fn part2(m: &Input) -> i64 {
    let mut mb = m.clone();
    mb.insert("humn".to_string(), Monkey::Operation("nul".to_string(), "+".to_string(), "nul".to_string()));
    let mut changed = true;
    while changed {
        changed = false;
        for (key,_) in m {
            if key == "humn" {
                continue;
            }
            if let Monkey::Operation(s1,op,s2) = &mb[key] {
                if let Monkey::Number(v1) = mb[s1] {
                    if let Monkey::Number(v2) = mb[s2] {
                        let new_val = match op.as_str() {
                            "+" => {v1 + v2},
                            "-" => {v1 - v2},
                            "*" => {v1 * v2},
                            _ => {v1 / v2}
                        };
                        mb.insert(key.to_string(), Monkey::Number(new_val));
                        changed = true;
                    }
                }
            }
        }
    }

    let mut l = "";
    let mut r = "";
    if let Monkey::Operation(a,_,c) = &mb["root"] {
        l = a.as_str();
        r = c.as_str();
    }
    let mut new_op_str = "";
    let mut cur_num = 0;
    if let Monkey::Number(val) = mb[l] {
        cur_num = val;
        new_op_str = r;
    } else if let Monkey::Number(val) = mb[r] {
        cur_num = val;
        new_op_str = l;
    }
    let mut s1 = "";
    let mut s2 = "";
    let mut op = "";
    loop {
        if let Monkey::Operation(a,b,c) = &mb[new_op_str] {
            s1 = a.as_str();
            s2 = c.as_str();
            op = b.as_str();
        }
        if let Monkey::Number(val) = mb[s1] {
            match op {
                "*" => {
                    cur_num /= val;
                },
                "+" => {
                    cur_num -= val;
                }
                "-" => {
                    cur_num -= val;
                    cur_num *= -1;
                }
                _ => { println!("PROBLEM"); }
            }
            new_op_str = s2;
        } else if let Monkey::Number(val) = mb[s2] {
            match op {
                "*" => {
                    cur_num /= val;
                },
                "+" => {
                    cur_num -= val;
                }
                "/" => {
                    cur_num *= val;
                }
                "-" => {
                    cur_num += val;
                }
                _ => { println!("PROBLEM"); }
            }
            new_op_str = s1;
        }
        if new_op_str == "humn" {
            return cur_num;
        }
    }
}

pub fn main() {
    let inst = parse_input("./input/day21/input.txt");
    
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
    fn day21_test1() {
        let inst = parse_input("./input/day21/test.txt");
        assert_eq!(part1(&inst), 152);
	}

    #[test]
    fn day21_test2() {
        let inst = parse_input("./input/day21/test.txt");
        assert_eq!(part2(&inst), 301);
	}
}
