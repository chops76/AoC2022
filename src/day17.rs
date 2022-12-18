use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::collections::HashMap;

struct Piece {
    width: i64,
    height: i64,
    points: Vec<(i64,i64)>
}

type Input = Vec<char>;

fn parse_input(path: &str) -> Vec<char> {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    fstr.chars().collect()
}

fn print_grid(grid: &Vec<Vec<char>>) {
    for y in 1..=grid.len() {
        for x in 0..grid[0].len() {
            print!("{}", grid[grid.len() - y][x])
        }
        println!();
    }
}

fn part1(jets: &Input) -> usize {
    let pieces:[Piece;5] = 
        [Piece { width: 4, height: 1, points: vec![(0,0),(1,0),(2,0),(3,0)]},
        Piece { width: 3, height: 3, points: vec![(0,1),(1,1),(1,2),(1,0),(2,1)]},
        Piece { width: 3, height: 3, points: vec![(0,0),(1,0),(2,0),(2,1),(2,2)]},
        Piece { width: 1, height: 4, points: vec![(0,0),(0,1),(0,2),(0,3)]},
        Piece { width: 2, height: 2, points: vec![(0,0),(1,0),(1,1),(0,1)]}];
    let mut piece = 0;
    let mut jet = 0;
    let mut grid = Vec::new();

    for _ in 0..2022 {
        // Add buffer
        for _ in 0..3 {
            grid.push(vec!['.';7]);
        }
        // Place part
        let mut ll = (2,grid.len());
        for _ in 0..pieces[piece].height {
            grid.push(vec!['.';7]);
        }
        //println!("Placing part {}", piece);

        loop {
            // Adjust for jet
            let check_x;
            if jets[jet] == '>' {
                //println!("Moving right?");
                check_x = ll.0 + 1;
            } else {
                //println!("Moving left?");
                check_x = ll.0 - 1;
            }
            let mut can_shift = true;
            for point in &pieces[piece].points {
                if point.0 + check_x > 6 || point.0 + check_x < 0 || 
                    grid[ll.1+point.1 as usize][check_x as usize+point.0 as usize] == '#' {
                        can_shift = false;
                        break;
                    }
            }
            if can_shift {
                //println!("Yes!");
                ll.0 = check_x;
            }

            //Drop piece
            let check_y:i64 = ll.1 as i64 - 1;
            let mut can_drop = true;
            for point in &pieces[piece].points {
                if check_y < 0 || 
                    grid[check_y as usize+point.1 as usize][ll.0 as usize+point.0 as usize] == '#' {
                        can_drop = false;
                        break;
                    }
            }
            jet += 1;
            jet %= jets.len();
            if can_drop {
                ll.1 = check_y as usize;
                //println!("Dropping");
            } else {
                //println!("Frozen");
                break;

            }
        }

        for point in &pieces[piece].points {
            grid[ll.1+point.1 as usize][ll.0 as usize+point.0 as usize] = '#';
        }
        while !grid[grid.len() -1].contains(&'#') {
            grid.pop();
        }
        piece += 1;
        piece %= pieces.len();
    }
    //print_grid(&grid);
    grid.len()
}

fn part2(jets: &Input) -> i64 {
    let pieces:[Piece;5] = 
        [Piece { width: 4, height: 1, points: vec![(0,0),(1,0),(2,0),(3,0)]},
        Piece { width: 3, height: 3, points: vec![(0,1),(1,1),(1,2),(1,0),(2,1)]},
        Piece { width: 3, height: 3, points: vec![(0,0),(1,0),(2,0),(2,1),(2,2)]},
        Piece { width: 1, height: 4, points: vec![(0,0),(0,1),(0,2),(0,3)]},
        Piece { width: 2, height: 2, points: vec![(0,0),(1,0),(1,1),(0,1)]}];
    let mut piece = 0;
    let mut jet = 0;
    let mut grid = Vec::new();
    let mut lengths:HashMap<usize, Vec<(i32,usize)>> = HashMap::new();

    for count in 0..3325 {
        // Add buffer
        for _ in 0..3 {
            grid.push(vec!['.';7]);
        }
        // Place part
        let mut ll = (2,grid.len());
        for _ in 0..pieces[piece].height {
            grid.push(vec!['.';7]);
        }
        //println!("Placing part {}", piece);

        loop {
            // Adjust for jet
            let check_x;
            if jets[jet] == '>' {
                //println!("Moving right?");
                check_x = ll.0 + 1;
            } else {
                //println!("Moving left?");
                check_x = ll.0 - 1;
            }
            let mut can_shift = true;
            for point in &pieces[piece].points {
                if point.0 + check_x > 6 || point.0 + check_x < 0 || 
                    grid[ll.1+point.1 as usize][check_x as usize+point.0 as usize] == '#' {
                        can_shift = false;
                        break;
                    }
            }
            if can_shift {
                //println!("Yes!");
                ll.0 = check_x;
            }

            //Drop piece
            let check_y:i64 = ll.1 as i64 - 1;
            let mut can_drop = true;
            for point in &pieces[piece].points {
                if check_y < 0 || 
                    grid[check_y as usize+point.1 as usize][ll.0 as usize+point.0 as usize] == '#' {
                        can_drop = false;
                        break;
                    }
            }
            jet += 1;
            jet %= jets.len();
            if can_drop {
                ll.1 = check_y as usize;
                //println!("Dropping");
            } else {
                //println!("Frozen");
                break;

            }
        }

        for point in &pieces[piece].points {
            grid[ll.1+point.1 as usize][ll.0 as usize+point.0 as usize] = '#';
        }
        while !grid[grid.len() -1].contains(&'#') {
            grid.pop();
        }
        piece += 1;
        piece %= pieces.len();
        if piece == 0 {
            if let Some(x) = lengths.get_mut(&jet) {
                x.push((count,grid.len()));
            } else {
                let mut tmp = Vec::new();
                tmp.push((count,grid.len()));
                lengths.insert(jet, tmp);
            }
        }
    }
    //print_grid(&grid);
    //println!("{:?}", lengths);
    // I used lengths to discover that at 1889 pieces, the length is 2985.  
    // Also, at 3614 pieces, the length is 5694, and at 5339, we get 8403.  Every 1725
    // pieces after the first 1889 we add 2709 in grid length.  Therefore we should be able
    // to calculate the length as below.
    let num_chunks:i64 = 1000000000000 / 1725;
    let to_calc:i64 = 1000000000000 - (num_chunks - 1) * 1725;  // This is where the 3325 above came from
    //println!("To calc: {}", to_calc);
    grid.len() as i64 + ((num_chunks - 1) * 2709)
}

pub fn main() {
    let jets = parse_input("./input/day17/input.txt");
    
    let p1_timer = Instant::now();
    let p1_result = part1(&jets);
    let p1_time = p1_timer.elapsed();
    println!("Part 1: {}", p1_result);
    println!("Part 1 Time: {:?}", p1_time);

    let p2_timer = Instant::now();
    let p2_result = part2(&jets);
    let p2_time = p2_timer.elapsed();
    println!("Part 2: {}", p2_result);
    println!("Part 2 Time: {:?}", p2_time);  
}

