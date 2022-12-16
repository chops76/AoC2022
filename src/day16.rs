use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

type Input = Vec<(String, i64, Vec<String>)>;

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    let mut ret_vec: Input = Vec::new();
    for l in BufReader::new(f).lines().flatten() {
        let spl = l.split_whitespace().collect::<Vec<&str>>();
        let key = spl[1];
        let release: i64 = spl[4][5..spl[4].len()-1].parse().unwrap();
        let spl2 = if l.contains("valves") 
            { l.split(" valves ").collect::<Vec<&str>>() } else 
            { l.split(" valve ").collect::<Vec<&str>>() };
        let paths = spl2[1].split(", ").map(|s| s.to_string()).collect();
        ret_vec.push((key.to_string(), release, paths));
    }
    ret_vec
}

fn find_energy(start_loc: usize, cur_valve: usize, visited: &Vec<bool>, time: i64, valves: &Vec<usize>, sum: i64, rooms: &Input, graph: &Vec<Vec<i64>>, total_time: i64) -> i64 {
    let valve_idx;
    if cur_valve == 999 {
        valve_idx = start_loc;
    } else {
        valve_idx = valves[cur_valve];
    }
    let mut new_visited = visited.clone();
    if cur_valve != 999 {
        new_visited[cur_valve] = true;  
    }
    if time > total_time {
        return sum;
    }
    let sum = sum + (total_time - time) * rooms[valve_idx].1;

    let mut best = sum;
    for i in 0..valves.len() {
        if new_visited[i] {
            continue;
        }
        let e = find_energy(start_loc, i, &new_visited, time + graph[valve_idx][valves[i]] + 1, valves, sum, rooms, graph, total_time);
        if e > best {
            best = e;
        }

    }
    best
}

fn part1(rooms: &Input) -> i64 {
    let mut lookup = HashMap::new();
    for i in 0..rooms.len() {
        lookup.insert(rooms[i].0.clone(), i);
    }
    let inf:i64 = 99999;
    let mut graph = vec![vec![inf;rooms.len()];rooms.len()];
    for i in 0..rooms.len() {
        graph[i][i] = 0;
        for j in 0..rooms[i].2.len() {
            graph[i][*lookup.get(&rooms[i].2[j]).unwrap()] = 1;
        }
    }

    for k in 0..graph.len() {
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                if graph[i][j] > (graph[i][k] + graph[k][j])
                && graph[k][j] != inf && graph[i][k] != inf {
                    graph[i][j] = graph[i][k] + graph[k][j];
                }
            }
        }
    }

    let valves = rooms.iter().filter(|(_,p,_)| *p != 0).map(|(s,_,_)| *lookup.get(s).unwrap()).collect::<Vec<usize>>();
    find_energy(*lookup.get("AA").unwrap(), 999, &vec![false; valves.len()], 0, &valves, 0, rooms, &graph, 30)
}

fn subsets<T: Clone>(items: Vec<T>) -> Vec<Vec<T>> {
    (0..=items.len())
        .map(|count| items.clone().into_iter().combinations(count))
        .flatten()
        .collect()
}

fn part2(rooms: &Input) -> i64 {
    let mut lookup = HashMap::new();
    for i in 0..rooms.len() {
        lookup.insert(rooms[i].0.clone(), i);
    }
    let inf:i64 = 99999;
    let mut graph = vec![vec![inf;rooms.len()];rooms.len()];
    for i in 0..rooms.len() {
        graph[i][i] = 0;
        for j in 0..rooms[i].2.len() {
            graph[i][*lookup.get(&rooms[i].2[j]).unwrap()] = 1;
        }
    }

    for k in 0..graph.len() {
        for i in 0..graph.len() {
            for j in 0..graph.len() {
                if graph[i][j] > (graph[i][k] + graph[k][j])
                && graph[k][j] != inf && graph[i][k] != inf {
                    graph[i][j] = graph[i][k] + graph[k][j];
                }
            }
        }
    }

    let valves = rooms.iter().filter(|(_,p,_)| *p != 0).map(|(s,_,_)| *lookup.get(s).unwrap()).collect::<Vec<usize>>();
    let it = subsets(valves.clone());
    let mut best = 0;
    let valve_set = valves.iter().collect::<HashSet<&usize>>();
    for v in it {
        let vset = v.iter().collect::<HashSet<&usize>>();
        let elephant = valve_set.difference(&vset).map(|v| **v).collect::<Vec<usize>>();

        let e = 
            find_energy(*lookup.get("AA").unwrap(), 999, &vec![false; v.len()], 0, &v, 0, rooms, &graph, 26) +
            find_energy(*lookup.get("AA").unwrap(), 999, &vec![false; elephant.len()], 0, &elephant, 0, rooms, &graph, 26);
        if e > best {
            best = e;
        }
    }
    
    best
}

pub fn main() {
    let inst = parse_input("./input/day16/input.txt");
    
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
    fn day14_test1() {
        let inst = parse_input("./input/day16/test.txt");
        assert_eq!(part1(&inst), 1651);
	}

    #[test]
    fn day14_test2() {
        let inst = parse_input("./input/day16/test.txt");
        assert_eq!(part2(&inst), 1707);
	}
}
