use std::io::{BufRead, BufReader};
use std::fs::File;
use std::time::Instant;

type Input = Vec<String>;

#[derive(Debug)]
struct FileEntry {
    name: String,
    size: usize
}

#[derive(Debug)]
struct Directory {
    name: String,
    parent: usize,
    files: Vec<usize>,
    subdirs: Vec<usize>
}

fn parse_input(filename: &str) -> Input {
    let f = File::open(filename).unwrap();
    BufReader::new(f).lines().flatten().collect()
}

fn calc_size(dir_num: usize, dirs: &Vec<Directory>, files: &Vec<FileEntry>) -> usize {
    let mut sum = 0;
    for d in &dirs[dir_num].subdirs {
        sum += calc_size(*d, dirs, files);
    }
    for f in &dirs[dir_num].files {
        sum += files[*f].size;
    }
    sum
}

fn create_tree(terminal: &Input) -> (Vec<Directory>,Vec<FileEntry>) {
    let mut dirs: Vec<Directory> = Vec::new();
    let mut files: Vec<FileEntry> = Vec::new();
    dirs.push(Directory { name: "/".to_string(), parent: 0, files: Vec::new(), subdirs: Vec::new() });
    let mut cur_dir = 0;
    let mut cur_line = 0;
    let mut last_dir = 1;
    let mut last_file = 0;
    while cur_line < terminal.len() {
        let spl = terminal[cur_line].split_ascii_whitespace().collect::<Vec<&str>>();
        if spl[0] == "$" {
            if spl[1] == "cd" {
                if spl[2] == "/" {
                    cur_dir = 0;
                } else if spl[2] == ".." {
                    cur_dir = dirs[cur_dir].parent;
                } else {
                    for d in &dirs[cur_dir].subdirs {
                        if dirs[*d].name == spl[2] {
                            cur_dir = *d;
                            break;
                        }
                    }
                }
            }
        } else {
            if spl[0] == "dir" { 
                let new_dir = Directory { name: spl[1].to_string(), parent: cur_dir, 
                                                        files: Vec::new(), subdirs: Vec::new()};
                dirs.push(new_dir);
                dirs[cur_dir].subdirs.push( last_dir );
                last_dir += 1;
            } else {
                let new_entry = FileEntry { name: spl[1].to_string(), size: spl[0].parse().unwrap() };
                files.push( new_entry );
                dirs[cur_dir].files.push(last_file);
                last_file += 1;
            }
        }
        cur_line += 1;
    }
    (dirs,files)
}

fn part1(terminal: &Input) -> usize {
    let (dirs,files) = create_tree(terminal);
    let mut total = 0;
    for d in 0..dirs.len() {
        let sum = calc_size(d, &dirs, &files);
        if sum <= 100000 {
            total += sum;
        }
    } 
    total
}

fn part2(terminal: &Input) -> usize {
    let (dirs,files) = create_tree(terminal);

    let root_size = calc_size(0, &dirs, &files);
    let free = 70000000 - root_size;
    let needed = 30000000 - free;
    let mut best = root_size;
    for d in 0..dirs.len() {
        let sum = calc_size(d, &dirs, &files);
        if sum > needed && sum < best {
            best = sum;
        }
    }    
    best
}

pub fn main() {
    let terminal = parse_input("./input/day7/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&terminal);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&terminal);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time); 
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn day7_test1() {
        let terminal = parse_input("./input/day7/test.txt");
        assert_eq!(part1(&terminal), 95437);
	}

    #[test]
    fn day7_test2() {
        let terminal = parse_input("./input/day7/test.txt");
        assert_eq!(part2(&terminal), 24933642);
	}

}
