use std::io::Read;
use std::fs::File;
use std::time::Instant;

#[derive(Debug)]
enum Dir {
    Right,
    Left,
    Forward(usize)
}

type Input = (Vec<Vec<char>>, Vec<Dir>);

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    let spl = fstr.split("\n\n").collect::<Vec<&str>>();
    let grid = spl[0].split("\n").map(|l| l.chars().collect::<Vec<char>>()).collect();
    let chars = spl[1].chars().collect::<Vec<char>>();
    let mut dirs = Vec::new();
    let mut cur_pos = 0;
    while cur_pos < spl[1].len() {
        if chars[cur_pos] == 'R' {
            dirs.push(Dir::Right);
            cur_pos += 1;
        } else if chars[cur_pos] == 'L' {
            dirs.push(Dir::Left);
            cur_pos += 1;
        } else {
            let mut new_pos = cur_pos + 1;
            while new_pos < spl[1].len() && chars[new_pos] != 'L' && chars[new_pos] != 'R' {
                new_pos += 1;
            }
            dirs.push(Dir::Forward(spl[1][cur_pos..new_pos].parse().unwrap()));
            cur_pos = new_pos;
        }
    }

    (grid,dirs)
}

fn travel(x: usize, y: usize, facing: char, grid: &Vec<Vec<char>>) -> (usize,usize) {
    match facing {
        'R' => {
            let mut new_x = x + 1;
            if new_x == grid[y].len() || grid[y][new_x] == ' ' {
                new_x = grid[y].iter().position(|&x| x != ' ').unwrap();
            }
            (new_x, y)
        },
        'L' => {
            let mut new_x:i32 = x as i32 - 1;
            if new_x < 0 || grid[y][new_x as usize] == ' ' {
                new_x = grid[y].len() as i32 - 1;
                while grid[y][new_x as usize] == ' ' {
                    new_x -= 1;
                }
            }
            (new_x as usize, y)
        },
        'D' => {
            let mut new_y = y + 1;
            if new_y == grid.len() || grid[new_y].len() <= x || grid[new_y][x] == ' ' {
                new_y = 0;
                while grid[new_y].len() <= x || grid[new_y][x] == ' ' {
                    new_y += 1;
                }
            }
            (x, new_y)
        },
        'U' => {
            let mut new_y:i32 = y as i32 - 1;
            if new_y < 0 || grid[new_y as usize].len() <= x || grid[new_y as usize][x] == ' ' {
                new_y = grid.len() as i32 - 1;
                while grid[new_y as usize].len() <= x || grid[new_y as usize][x] == ' ' {
                    new_y -= 1;
                }
            }
            (x, new_y as usize)
        }
        _ => {
            println!("Illegal Direction");
            (x,y)
        }
    }
}

fn cube_travel(x: usize, y: usize, facing: char) -> (usize,usize,char) {
    match facing {
        'R' => {
            let mut new_x = x + 1;
            let mut new_y = y;
            let mut new_facing = facing;
            if new_x == 150 {
                if new_y <= 50 {
                    new_facing = 'L';
                    new_x = 99;
                    new_y = 149 - new_y;
                }
            } else if new_x == 100 {
                if new_y >= 50 && new_y <= 99 {
                    new_facing = 'U';
                    new_x = 50 + new_y;
                    new_y = 49;
                } else if new_y >= 100 && new_y <= 149 {
                    new_facing = 'L';
                    new_x = 149;
                    new_y = 149 - new_y;
                }
            } else if new_x == 50 {
                if new_y >= 150 && new_y <= 199 {
                    new_facing = 'U';
                    new_x = new_y - 100;
                    new_y = 149;
                }
            }
            (new_x, new_y, new_facing)
        },
        'L' => {
            let mut new_x:i32 = x as i32 - 1;
            let mut new_y = y;
            let mut new_facing = facing;
            if new_x == 49 {
                if new_y <= 49 {
                    new_facing = 'R';
                    new_x = 0;
                    new_y = 149 - new_y;
                } else if new_y >= 50 && new_y <= 99 {
                    new_facing = 'D';
                    new_x = new_y as i32 - 50;
                    new_y = 100;
                }
            } else if new_x == -1 {
                if new_y >= 100 && new_y <= 149 {
                    new_facing = 'R';
                    new_x = 50;
                    new_y = 149 - new_y;
                } else if new_y >= 150 && new_y <= 199 {
                    new_facing = 'D';
                    new_x = new_y as i32 - 100;
                    new_y = 0;
                }
            }
            (new_x as usize, new_y, new_facing)
        },
        'D' => {
            let mut new_x = x;
            let mut new_y = y + 1;
            let mut new_facing = facing;
            if new_y == 200 && new_x <=49 {
                 new_x += 100;
                 new_y = 0;
            } else if new_y == 150 && new_x >= 50 && new_x <= 99 {
                new_facing = 'L';
                new_y = new_x + 100;
                new_x = 49;
            } else if new_y == 50 && new_x >= 100 && new_x <= 149 {
                new_facing = 'L';
                new_y = new_x - 50;
                new_x = 99;
            }
            (new_x, new_y, new_facing)
        },
        'U' => {
            let mut new_x = x;
            let mut new_y:i32 = y as i32 - 1;
            let mut new_facing = facing;
            if new_y == 99 && new_x <= 49 {
                new_facing = 'R';
                new_y = 50 + new_x as i32;
                new_x = 50;
            } else if new_y == -1 && new_x >= 50 && new_x <= 99 {
                new_facing = 'R';
                new_y = new_x as i32 + 100;
                new_x = 0;
            } else if new_y == -1 && new_x >= 100 && new_x <= 149 {
                new_x = new_x - 100;
                new_y = 199;
            }
            (new_x, new_y as usize, new_facing)
        }
        _ => {
            println!("Illegal Direction");
            (x,y,facing)
        }
    }
}

fn part1(grid: &Vec<Vec<char>>, dirs: &Vec<Dir>) -> i64 {
    let mut facing = 'R';
    let mut x = grid[0].iter().position(|&x| x == '.').unwrap();
    let mut y = 0;
    for dir in dirs {
        match dir {
            Dir::Forward(num) => {
                for _ in 0..*num {
                    let (new_x, new_y) = travel(x, y, facing, grid);
                    if grid[new_y][new_x] == '#' {
                        break;
                    }
                    x = new_x;
                    y = new_y;    
                }  
            },
            Dir::Right => {
                facing = match facing {
                    'R' => 'D', 'D' => 'L', 'L' => 'U', 'U' => 'R', _ => 'X'
                };
            },
            Dir::Left => {
                facing = match facing {
                    'R' => 'U', 'U' => 'L', 'L' => 'D', 'D' => 'R', _ => 'X'
                };
            }
        }
    }
    1000 * (y + 1) as i64 + 4 * (x + 1) as i64 + match facing { 'R' => 0, 'D' => 1, 'L' => 2, 'U' => 3, _ => 0 }
}

fn part2(grid: &Vec<Vec<char>>, dirs: &Vec<Dir>) -> i64 {
    let mut facing = 'R';
    let mut x = grid[0].iter().position(|&x| x == '.').unwrap();
    let mut y = 0;
    for dir in dirs {
        match dir {
            Dir::Forward(num) => {
                for _ in 0..*num {
                    let (new_x, new_y, new_facing) = cube_travel(x, y, facing);
                    if grid[new_y][new_x] == '#' {
                        break;
                    } 
                    x = new_x;
                    y = new_y;    
                    facing = new_facing;
                }  
            },
            Dir::Right => {
                facing = match facing {
                    'R' => 'D', 'D' => 'L', 'L' => 'U', 'U' => 'R', _ => 'X'
                };
            },
            Dir::Left => {
                facing = match facing {
                    'R' => 'U', 'U' => 'L', 'L' => 'D', 'D' => 'R', _ => 'X'
                };
            }
        }
    }
    1000 * (y + 1) as i64 + 4 * (x + 1) as i64 + match facing { 'R' => 0, 'D' => 1, 'L' => 2, 'U' => 3, _ => 0 }

}

pub fn main() {
    let (grid,dirs) = parse_input("./input/day22/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&grid, &dirs);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&grid, &dirs);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

