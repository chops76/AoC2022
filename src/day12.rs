use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

type Input = Vec<Vec<usize>>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> Vec<usize> {
    line.chars().map(|c| match c { 'S' => 100, 'E' => 0, _ => c as usize - 96 }).collect()
}

fn find_endpoints(grid: &Input) -> (usize,usize,usize,usize) {
    let mut xs = 0;
    let mut ys = 0;
    let mut xf = 0;
    let mut yf = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 0 {
                xf = j;
                yf = i;
            } else if grid[i][j] == 100 {
                xs = j;
                ys = i;
            }
        }
    }
    (xs,ys,xf,yf)
}

fn find_shortest(grid: &Input, xs: usize, ys: usize, xf: usize, yf: usize) -> usize {
    let mut unvisited = HashSet::new();
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            unvisited.insert((x, y));
        }
    }

    let mut queue = VecDeque::new();
    queue.push_front(((xs,ys),0));

    while let Some(((xc,yc),cost)) = queue.pop_front() {
        if unvisited.contains(&(xc,yc)) {
            unvisited.remove(&(xc,yc));

            if xc == xf && yc == yf {
                return cost;
            }
            if xc > 0 && grid[yc][xc - 1] as i32 - grid[yc][xc] as i32 <= 1 {
                queue.push_back(((xc - 1,yc),cost+1));
            }
            if xc < grid[0].len() - 1 && grid[yc][xc + 1] as i32 - grid[yc][xc] as i32 <= 1 {
                queue.push_back(((xc + 1,yc),cost+1));
            }   
            if yc > 0 && grid[yc - 1][xc] as i32 - grid[yc][xc] as i32 <= 1 {
                queue.push_back(((xc,yc - 1),cost+1));
            }
            if yc < grid.len() - 1 && grid[yc + 1][xc] as i32 - grid[yc][xc] as i32 <= 1 {
                queue.push_back(((xc,yc + 1),cost+1)); 
            }
        }
    }
    100000
}

fn part1(grid_data: &Input) -> usize {
    let mut grid = grid_data.clone();
    let (xs,ys,xf,yf) = find_endpoints(&grid);
    grid[ys][xs] = 1;
    grid[yf][xf] = 26;

    find_shortest(&grid, xs, ys, xf, yf)
}

fn part2(grid_data: &Input) -> usize {
    let mut grid = grid_data.clone();
    let (xs,ys,xf,yf) = find_endpoints(&grid);
    grid[ys][xs] = 1;
    grid[yf][xf] = 26;

    let mut lowest = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 1 {
                lowest.push((j,i));
            }
        }
    }

    lowest.iter().map(|(x,y)| find_shortest(&grid,*x,*y,xf,yf)).min().unwrap()
}

pub fn main() {
    let inst = parse_input("./input/day12/input.txt");
    
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
    fn day12_test1() {
        let inst = parse_input("./input/day12/test.txt");
        assert_eq!(part1(&inst), 31);
	}

    #[test]
    fn day12_test2() {
        let inst = parse_input("./input/day12/test.txt");
        assert_eq!(part2(&inst), 29);
	}
}
