use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<i64>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|l| l.parse().unwrap()).collect()
}

fn mix(table: &mut Vec<(i64,usize,usize)>) {
    for i in 0..table.len() {
        let mut prev = table[i].2;
        let mut next = table[i].1;
        table[prev].1 = next;
        table[next].2 = prev;
        if table[i].0 >= 0 {
            for _ in 0..(table[i].0 % (table.len() as i64 - 1)) {
                prev = table[prev].1;
            }
            next = table[prev].1;
        } else {
            for _ in 0..(table[i].0.abs() % (table.len() as i64 - 1)) {
                next = table[next].2;
            }
            prev = table[next].2;
        }
        table[i].1 = next;
        table[i].2 = prev;
        table[next].2 = i;
        table[prev].1 = i;
    }
}

fn calc_sum(table: &Vec<(i64,usize,usize)>) -> i64 {
    let mut idx = 0;
    for i in 0..table.len() {
        if table[i].0 == 0 {
            idx = i;
            break;
        }
    }
    for _ in 0..1000 {
        idx = table[idx].1;
    }
    let mut sum = table[idx].0;
    for _ in 0..1000 {
        idx = table[idx].1;
    }
    sum += table[idx].0;
    for _ in 0..1000 {
        idx = table[idx].1;
    }
    sum += table[idx].0;   
    sum
}

fn part1(numbers: &Input) -> i64 {
    let mut table:Vec<(i64,usize,usize)> = 
        numbers.iter().enumerate()
            .map(|(i,v)| (*v,(i+1)%numbers.len(),(i+numbers.len()-1)%numbers.len()))
            .collect();
    mix(&mut table);
    calc_sum(&table)
}

fn part2(numbers: &Input) -> i64 {
    let key:i64 = 811589153;
    let mut table:Vec<(i64,usize,usize)> = 
        numbers.iter().enumerate()
            .map(|(i,v)| (*v * key,(i+1)%numbers.len(),(i+numbers.len()-1)%numbers.len()))
            .collect();
    for _ in 0..10 {
        mix(&mut table);
    }
    calc_sum(&table)
}

pub fn main() {
    let inst = parse_input("./input/day20/input.txt");
    
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
    fn day20_test1() {
        let inst = parse_input("./input/day20/test.txt");
        assert_eq!(part1(&inst), 3);
	}

    #[test]
    fn day20_test2() {
        let inst = parse_input("./input/day20/test.txt");
        assert_eq!(part2(&inst), 1623178306);
	}
}
