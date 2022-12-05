use std::io::Read;
use std::fs::File;
use std::time::Instant;

type Input = (Vec<Vec<char>>,Vec<(usize,usize,usize)>);

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    let spl = fstr.split("\n\n").collect::<Vec<&str>>();
    let stack = spl[0].split("\n").collect::<Vec<&str>>();
    let num_stacks = 
        (stack[stack.len() - 1].chars()
            .nth(stack[stack.len() - 1].len() - 2).unwrap() as u32 - 48) as usize;

    let mut stack_vec = Vec::new();
    for i in 0..num_stacks {
        stack_vec.push(Vec::new());
    }
    for i in 0..stack.len() - 1 {
        for j in 0..num_stacks {
            let item = stack[i].chars().nth(1 + j * 4).unwrap();
            if item != ' ' {
                stack_vec[j].push(item);
            }
        }
    }
    for i in 0..num_stacks {
        stack_vec[i].reverse();
    }

    let inst = spl[1].split('\n').map(|s| parse_line(s)).collect();
    (stack_vec, inst)
}

fn parse_line(line: &str) -> (usize,usize,usize) {
    let spl:Vec<&str> = line.split(' ').collect();

    (spl[1].parse().unwrap(),spl[3].parse().unwrap(),spl[5].parse().unwrap())
}

fn part1(stacks: &Vec<Vec<char>>, inst: &Vec<(usize,usize,usize)>) -> String {
    let mut mstacks = stacks.clone();

    for (num, from, to) in inst {
        for _ in 0..*num {
            let c = mstacks[*from - 1].pop().unwrap();
            mstacks[*to - 1].push(c);
        }
    }
    mstacks.iter().map(|v| v.iter().last().unwrap()).collect::<String>()
}

fn part2(stacks: &Vec<Vec<char>>, inst: &Vec<(usize,usize,usize)>) -> String {
    let mut mstacks = stacks.clone();

    for (num, from, to) in inst {
        let mut tmp = Vec::new();
        for _ in 0..*num {
            let c = mstacks[*from - 1].pop().unwrap();
            tmp.push(c);
        }
        for _ in 0..*num {
            let c = tmp.pop().unwrap();
            mstacks[*to - 1].push(c);
        }
    }
    mstacks.iter().map(|v| v.iter().last().unwrap()).collect::<String>()
}

pub fn main() {
    let (stacks, inst) = parse_input("./input/day5/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&stacks, &inst);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&stacks, &inst);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day1_test1() {
        let (stacks, inst) = parse_input("./input/day5/test.txt");
        assert_eq!(part1(&stacks, &inst), "CMZ");
	}

    #[test]
    fn day1_test2() {
        let (stacks, inst) = parse_input("./input/day5/test.txt");
        assert_eq!(part2(&stacks, &inst), "MCD");
	}
}
