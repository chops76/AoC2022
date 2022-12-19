use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::VecDeque;
use std::collections::HashSet;

type Input = Vec<(i64,i64,(i64,i64),(i64,i64))>;

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Hash)]
#[derive(Debug)]
#[derive(Clone)]
struct Status {
    ore_robots: i64,
    clay_robots: i64,
    obsidian_robots: i64,
    geode_robots: i64,
    ore: i64,
    clay: i64,
    obsidian: i64,
    geodes: i64,
    time_left: i64
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().map(|s| parse_line(&s)).collect()
}

fn parse_line(line: &str) -> (i64,i64,(i64,i64),(i64,i64)) {
    let spl = line.split_whitespace().collect::<Vec<&str>>();

    (spl[6].parse().unwrap(), spl[12].parse().unwrap(), 
     (spl[18].parse().unwrap(),spl[21].parse().unwrap()),
     (spl[27].parse().unwrap(),spl[30].parse().unwrap()))
}

fn robot_work(bp: (i64,i64,(i64,i64),(i64,i64)), time: i64) -> i64 {
    let mut queue = VecDeque::new();
    queue.push_back(Status {ore_robots:1,clay_robots:0,obsidian_robots:0,geode_robots:0,
                            ore:0,clay:0,obsidian:0,geodes:0,time_left:time});                          
    let max_ore = *[bp.1,bp.2.0,bp.3.0].iter().max().unwrap();
    let max_clay = bp.2.1;
    let max_obsidian = bp.3.1;
    let mut seen = HashSet::new();
    let mut best = 0;
    while let Some(state) = queue.pop_front() {
        if !seen.contains(&state) {
            seen.insert(state.clone());
        } else {
            continue;
        }
        if state.time_left == 0 {
            best = best.max(state.geodes);
            continue;
        }
        let mut new_state = state.clone();
        new_state.ore += state.ore_robots;
        new_state.clay += state.clay_robots;
        new_state.obsidian += state.obsidian_robots;
        new_state.geodes += state.geode_robots;
        new_state.time_left -= 1;

        if state.ore >= bp.3.0 && state.obsidian >= bp.3.1 {
            let mut build_state = new_state.clone();
            build_state.ore -= bp.3.0;
            build_state.obsidian -= bp.3.1;
            build_state.geode_robots += 1;
            queue.push_back(build_state);    
            continue;    
        }
        if state.ore >= bp.0 && new_state.ore_robots < max_ore {
            let mut build_state = new_state.clone();
            build_state.ore -= bp.0;
            build_state.ore_robots += 1;
            queue.push_back(build_state);
        }
        if state.ore >= bp.1 && new_state.clay_robots < max_clay {
            let mut build_state = new_state.clone();
            build_state.ore -= bp.1;
            build_state.clay_robots += 1;
            queue.push_back(build_state);
        }
        if state.ore >= bp.2.0 && state.clay >= bp.2.1 && new_state.obsidian_robots < max_obsidian {
            let mut build_state = new_state.clone();
            build_state.ore -= bp.2.0;
            build_state.clay -= bp.2.1;
            build_state.obsidian_robots += 1;
            queue.push_back(build_state);        
        }
        queue.push_back(new_state);
    }

    best
}

fn part1(blueprints: &Input) -> i64 {
    let mut total_quality = 0;
    for i in 0..blueprints.len() {
        total_quality += (i + 1) as i64 * robot_work(blueprints[i], 24);
    }
    total_quality
}

fn part2(blueprints: &Input) -> i64 {
    let mut product = 1;
    for i in 0..3 {
        product *= robot_work(blueprints[i], 32);
    }
    product
}

pub fn main() {
    let inst = parse_input("./input/day19/input.txt");
    
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

