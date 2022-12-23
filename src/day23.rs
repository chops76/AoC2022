use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;

type Input = Vec<Vec<char>>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|l| l.chars().collect::<Vec<char>>()).collect()
}

fn create_set(grid: &Vec<Vec<char>>) -> HashSet<(i64,i64)> {
    let mut elves = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] == '#' {
                elves.insert((x as i64, y as i64));
            }
        }
    }
    elves
}

fn get_proposed((x,y): &(i64,i64), elves: &HashSet<(i64,i64)>, offset: usize) -> (i64,i64) {
    let all_dirs = [(x-1,y-1),(*x,y-1),(x+1,y-1),(x-1,*y),(x+1,*y),(x-1,y+1),(*x,y+1),(x+1,y+1)];
    if all_dirs.iter().all(|p| !elves.contains(p)) {
        return (*x,*y);
    }
    let dirs = vec!['N', 'S', 'W', 'E'];
    for i in 0..4 {
        let to_check = match dirs[(offset+i)%4] {
            'N' => [(x-1,y-1),(*x,y-1),(x+1,y-1)],
            'S' => [(x-1,y+1),(*x,y+1),(x+1,y+1)],
            'E' => [(x+1,y-1),(x+1,*y),(x+1,y+1)],
            'W' => [(x-1,y-1),(x-1,*y),(x-1,y+1)],
            _ => {println!("Illegal Direction"); [(*x,*y),(*x,*y),(*x,*y)]}
        };
        if to_check.iter().all(|p|!elves.contains(p)) {
            return match dirs[(offset+i)%4] {
                'N' => (*x,y-1),
                'S' => (*x,y+1),
                'E' => (x+1,*y),
                'W' => (x-1,*y),
                _ => (*x,*y)
            }
        }
    }        
    (*x,*y)
}

fn will_move(elves: &HashMap<(i64,i64),(i64,i64)>) -> HashMap<(i64,i64),(i64,i64)> {
    let mut count = HashMap::new();
    for (_,p) in elves {
        if !count.contains_key(&p) {
            count.insert(p,1);
        } else {
            count.insert(p, count[&p]+1);
        }
    }

    return elves.clone().into_iter()
        .filter(|&(_, v)| count[&v] == 1)
        .collect();
}

fn find_open_ground(elves: &HashSet<(i64,i64)>) -> i64 {
    let mut count = 0;
    let pos = elves.iter().next().unwrap();
    let mut min_x = pos.0;
    let mut max_x = pos.0;
    let mut min_y = pos.1;
    let mut max_y = pos.1;
    for e in elves {
        min_x = min_x.min(e.0);
        max_x = max_x.max(e.0);
        min_y = min_y.min(e.1);
        max_y = max_y.max(e.1);
    }
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !elves.contains(&(x,y)) {
                count += 1;
            }
        }
    }

    count
}

fn part1(grid: &Vec<Vec<char>>) -> i64 {
    let mut elves = create_set(grid);
    for i in 0..10 {
        let mut proposed = HashMap::new();
        for elf in &elves {
            let pos = get_proposed(elf, &elves, i % 4);
            proposed.insert(*elf, pos);
        }
        let will_move = will_move(&proposed);
        for m in will_move {
            elves.remove(&m.0);
            elves.insert(m.1);
        }
    }
    find_open_ground(&elves)
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let mut elves = create_set(grid);
    let mut count = 1;
    loop {
        let mut proposed = HashMap::new();
        let mut changed = false;
        for elf in &elves {
            let pos = get_proposed(elf, &elves, (count - 1) % 4);
            if pos != *elf {
                changed = true;
            }
            proposed.insert(*elf, pos);
        }
        if !changed {
            return count;
        }
        let will_move = will_move(&proposed);
        for m in will_move {
            elves.remove(&m.0);
            elves.insert(m.1);
        }
        count += 1;
    }
}

pub fn main() {
    let grid = parse_input("./input/day23/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&grid);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&grid);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day23_test1() {
        let inst = parse_input("./input/day23/test.txt");
        assert_eq!(part1(&inst), 110);
	}

    #[test]
    fn day23_test2() {
        let inst = parse_input("./input/day23/test.txt");
        assert_eq!(part2(&inst), 20);
	}
}