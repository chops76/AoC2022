use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

type Input = Vec<Vec<char>>;

fn parse_input(path: &str) -> Input {
    let f = File::open(path).unwrap();
    BufReader::new(f).lines().flatten().map(|l| l.chars().collect::<Vec<char>>()).collect()
}

fn get_blizzards(grid: &Vec<Vec<char>>) -> HashSet<(usize,usize,char)> {
    let mut blizzards = HashSet::new();
    for y in 1..grid.len() - 1 {
        for x in 1..grid[y].len() - 1 {
            if grid[y][x] != '.' {
                blizzards.insert((x,y,grid[y][x]));
            }
        }
    }
    blizzards
}

fn part1(grid: &Vec<Vec<char>>) -> i64 {
    let blizzards = get_blizzards(grid);
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let max_x = grid[0].len() - 2;
    let max_y = grid.len() - 2;
    queue.push_back((1, 0, 0));
    while let Some((x,y,steps)) = queue.pop_front() {
        if x == grid[y].len() - 2 && y == grid.len() - 1 {
            return steps;
        }
        if seen.contains(&(x, y, steps)) {
            continue;
        }
        seen.insert((x,y,steps));
        let mut collision = false;
        for b in &blizzards {
            let mut new_x = b.0;
            let mut new_y = b.1;
            match b.2 {
                '<' => {
                    let offset = steps as usize % max_x;
                    new_x = ((new_x - 1) + max_x - offset) % max_x + 1;
                },
                '>' => {
                    let offset = steps as usize % max_x;
                    new_x = ((new_x - 1) + offset) % max_x + 1;
                },
                '^' => {
                    let offset = steps as usize % max_y;
                    new_y = ((new_y - 1) + max_y - offset) % max_y + 1;
                },
                'v' => {
                    let offset = steps as usize % max_y;
                    new_y = ((new_y - 1) + offset) % max_y + 1;
                },
                _ => { println!("Illegal direction"); }
            }
            if x == new_x && y == new_y {
                collision = true;
                break;
            }
        }
        if collision {
            continue;
        }
        queue.push_back((x,y,steps+1));
        if y < max_y || x == max_x {
            queue.push_back((x,y+1,steps+1));
        }
        if y > 1 {
            queue.push_back((x,y-1,steps+1));
        }
        if x > 1 {
            queue.push_back((x-1,y,steps+1));
        }
        if x < max_x && y != 0 {
            queue.push_back((x+1,y,steps+1));
        }
    }
    0
}

fn part2(grid: &Vec<Vec<char>>) -> usize {
    let blizzards = get_blizzards(grid);
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let max_x = grid[0].len() - 2;
    let max_y = grid.len() - 2;
    queue.push_back((1, 0, 0, false, false));
    while let Some((x,y,steps, goal_seen, start_seen)) = queue.pop_front() {
        let mut new_goal_seen = goal_seen;
        let mut new_start_seen = start_seen;
        if seen.contains(&(x, y, steps,goal_seen,start_seen)) {
            continue;
        }
        seen.insert((x,y,steps,goal_seen,start_seen));
        if x == grid[y].len() - 2 && y == grid.len() - 1 {
            if start_seen {
                return steps;
            }
            new_goal_seen = true;
        }
        if x == 1 && y == 0 && new_goal_seen {
            new_start_seen = true;
        }
        let mut collision = false;
        for b in &blizzards {
            let mut new_x = b.0;
            let mut new_y = b.1;
            match b.2 {
                '<' => {
                    let offset = steps as usize % max_x;
                    new_x = ((new_x - 1) + max_x - offset) % max_x + 1;
                },
                '>' => {
                    let offset = steps as usize % max_x;
                    new_x = ((new_x - 1) + offset) % max_x + 1;
                },
                '^' => {
                    let offset = steps as usize % max_y;
                    new_y = ((new_y - 1) + max_y - offset) % max_y + 1;
                },
                'v' => {
                    let offset = steps as usize % max_y;
                    new_y = ((new_y - 1) + offset) % max_y + 1;
                },
                _ => { println!("Illegal direction"); }
            }
            if x == new_x && y == new_y {
                collision = true;
                break;
            }
        }
        if collision {
            continue;
        }

        queue.push_back((x,y,steps+1,new_goal_seen,new_start_seen));
        if y < max_y || (x == max_x && y == max_y){
            queue.push_back((x,y+1, steps+1,new_goal_seen,new_start_seen));
        }
        if y > 1 || (y == 1 && x == 1) {
            queue.push_back((x,y-1, steps+1,new_goal_seen,new_start_seen));
        }
        if x > 1 && y <= max_y {
            queue.push_back((x-1,y,steps+1,new_goal_seen,new_start_seen));
        }
        if x < max_x && y != 0 && y <= max_y {
            queue.push_back((x+1,y, steps+1,new_goal_seen,new_start_seen));
        }
    }
    0
}

pub fn main() {
    let grid = parse_input("./input/day24/input.txt");
    
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
        let inst = parse_input("./input/day24/test.txt");
        assert_eq!(part1(&inst), 18);
	}

    #[test]
    fn day23_test2() {
        let inst = parse_input("./input/day24/test.txt");
        assert_eq!(part2(&inst), 54);
	}
}