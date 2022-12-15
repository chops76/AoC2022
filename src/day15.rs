use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashSet;

type Input = Vec<((i64,i64),(i64,i64))>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> ((i64,i64),(i64,i64)) {
    let spl = line.split_whitespace().collect::<Vec<&str>>();

    ((spl[2][2..spl[2].len()-1].parse().unwrap(),spl[3][2..spl[3].len()-1].parse().unwrap()),
     (spl[8][2..spl[8].len()-1].parse().unwrap(),spl[9][2..].parse().unwrap()))
}

fn mdist((x1,y1): &(i64,i64),(x2,y2): &(i64,i64)) -> i64 {
    (x2-x1).abs() + (y2-y1).abs()
}

fn coverage_on_line((x,y): &(i64,i64), size: i64, line: i64) -> (i64,i64) {
    let radius = size - (line - y).abs();
    (x - radius,x + radius)
}

fn combine_ranges(r: &Vec<(i64,i64)>) -> Vec<(i64,i64)> {
    let mut ranges = r.clone();
    ranges.sort();
    let mut i = 0;
    let mut j = 0;

    while i < ranges.len() {
        if ranges[j].1 >= ranges[i].0 {
            ranges[j].1 = std::cmp::max(ranges[i].1,ranges[j].1);
        } else {
            j+=1;
            ranges[j]=ranges[i];
        }
        i+=1;
    }
    ranges.iter().take(j+1).map(|r| *r).collect()
}

fn find_ranges(points: &Input, line: i64) -> Vec<(i64,i64)> {
    let ranges = 
        points.iter().map(|(p1,p2)| (p1,mdist(p1,p2)))
                     .filter(|((_,y1),dist)| (y1-line).abs() <= *dist)     
                     .map(|(point,size)| coverage_on_line(point, size, line))      
                     .collect::<Vec<(i64,i64)>>();
    combine_ranges(&ranges)
}

fn part1(points: &Input, line: i64) -> i64 {
    let beacons_on_line: i64 = points.iter().filter(|(_,(_,y))| *y == line)
                                .map(|(_,p2)| p2)
                                .collect::<HashSet<&(i64,i64)>>().len() as i64;
    let combined = find_ranges(points, line);

    combined.iter().map(|(x1,x2)| (x2-x1) + 1).sum::<i64>() - beacons_on_line
}

fn part2(points: &Input, search_size: i64) -> i64 {
    for line in 0..=search_size {
        let ranges = find_ranges(points, line);
        if ranges.len() != 1 {
            return (ranges[0].1 + 1) * 4000000 + line;
        }
    }
    0
}

pub fn main() {
    let inst = parse_input("./input/day15/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&inst,2000000);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&inst, 4000000);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day14_test1() {
        let inst = parse_input("./input/day15/test.txt");
        assert_eq!(part1(&inst, 10), 26);
	}

    #[test]
    fn day14_test2() {
        let inst = parse_input("./input/day15/test.txt");
        assert_eq!(part2(&inst, 20), 56000011);
	}
}
