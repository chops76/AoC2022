use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<Vec<usize>>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> Vec<usize> {
    line.chars().map(|c| (c as u32 - 48) as usize).collect()
}

fn is_visible(x: i32, y: i32, dx: i32, dy: i32, trees: &Input) -> bool {
    let mut cx = x + dx;
    let mut cy = y + dy;

    while cx >= 0 && cy >= 0 && cx < trees[y as usize].len() as i32 && cy < trees.len() as i32 {
        if trees[cy as usize][cx as usize] >= trees[y as usize][x as usize] {
            return false;
        }
        cx += dx;
        cy += dy;
    }
    true
}

fn score(x: i32, y: i32, dx: i32, dy: i32, trees: &Input) -> i32 {
    let mut cx = x + dx;
    let mut cy = y + dy;
    let mut sum = 0;

    while cx >= 0 && cy >= 0 && cx < trees[y as usize].len() as i32 && cy < trees.len() as i32 {
        sum += 1;
        if trees[cy as usize][cx as usize] >= trees[y as usize][x as usize] {
            return sum;
        }
        cx += dx;
        cy += dy;
    }
    sum
}

fn part1(trees: &Input) -> usize {
    let mut total = 0;
    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            if is_visible(x as i32, y as i32, 1, 0, trees) ||
               is_visible(x as i32, y as i32, -1, 0, trees) ||
               is_visible(x as i32, y as i32, 0, 1, trees) ||
               is_visible(x as i32, y as i32, 0, -1, trees)
            {
                total += 1;
            }
        }
    }
    total
}

fn part2(trees: &Input) -> usize {
    let mut max_score = 0;
    for y in 0..trees.len() {
        for x in 0..trees[y].len() {
            let tree_score = 
                score (x as i32, y as i32, 1, 0, trees) *
                score(x as i32, y as i32, -1, 0, trees) *
                score(x as i32, y as i32, 0, 1, trees) *
                score(x as i32, y as i32, 0, -1, trees);
            max_score = std::cmp::max(max_score, tree_score);
        }
    }
    max_score as usize
}

pub fn main() {
    let trees = parse_input("./input/day8/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&trees);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&trees);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day8_test1() {
        let trees = parse_input("./input/day8/test.txt");
        assert_eq!(part1(&trees), 21);
	}

    #[test]
    fn day8_test2() {
        let trees = parse_input("./input/day8/test.txt");
        assert_eq!(part2(&trees), 8);
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
