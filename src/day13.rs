use std::io::Read;
use std::fs::File;
use std::time::Instant;
use std::cmp::Ordering;

type Input = Vec<(Vec<Item>,Vec<Item>)>;

#[derive(Debug)]
#[derive(Clone)]
enum Item {
    Integer(i32),
    ItemList(Vec<Item>)
}

fn parse_input(path: &str) -> Input {
    let mut fstr = String::new();

    File::open(path).unwrap().read_to_string(&mut fstr).unwrap();
    fstr.split("\n\n").map(|s| parse_pair(s)).collect()
}

fn parse_pair(line: &str) -> (Vec<Item>,Vec<Item>) {
    let spl = line.split("\n").collect::<Vec<&str>>();
    let mut cur_pos1 = 1;
    let mut cur_pos2 = 1;
    (parse_list(&spl[0],&mut cur_pos1),parse_list(&spl[1],&mut cur_pos2))
}

fn parse_list(line: &str, cur_pos: &mut usize) -> Vec<Item> {
    let mut ret_vec = Vec::new();
    let char_vec = line.chars().collect::<Vec<char>>();
    while char_vec[*cur_pos] != ']' && cur_pos < &mut line.len() {
        if char_vec[*cur_pos] == ',' {
            *cur_pos += 1;
        }
        if char_vec[*cur_pos].is_digit(10) {
            let mut new_pos = *cur_pos + 1;
            while char_vec[new_pos].is_digit(10) {
                new_pos += 1;
            }
            let val: i32 = line[*cur_pos..new_pos].parse().unwrap();
            *cur_pos = new_pos;
            ret_vec.push(Item::Integer(val));
        }
        if char_vec[*cur_pos] == '[' {
            *cur_pos += 1;
            ret_vec.push(Item::ItemList(parse_list(line, cur_pos)));
        }
    }
    *cur_pos += 1;
    ret_vec
}

fn compare_items(left: &Item, right: &Item) -> Ordering {
    if let Item::Integer(lv) = left {
        if let Item::Integer(rv) = right {
            return lv.cmp(rv);               
        }
        if let Item::ItemList(rlist) = right {
            let mut tmp_vec = Vec::new();
            tmp_vec.push(Item::Integer(*lv));
            return compare_lists(&tmp_vec, rlist);               
        }
    }
    if let Item::ItemList(llist) = left {
        if let Item::Integer(rv) = right {
            let mut tmp_vec = Vec::new();
            tmp_vec.push(Item::Integer(*rv));
            return compare_lists(llist, &tmp_vec);            
        }
        if let Item::ItemList(rlist) = right {
            return compare_lists(llist, rlist);               
        }
    }  
    Ordering::Equal  
}

fn compare_lists(left: &Vec<Item>, right: &Vec<Item>) -> Ordering {
    for count in 0..std::cmp::min(left.len(), right.len()) {
        let result = compare_items(&left[count], &right[count]);
        if result != Ordering::Equal {
            return result;
        }
    }
    left.len().cmp(&right.len())
}

fn create_marker(value: i32) -> Vec<Item> {
    let mut tmp_vec = Vec::new();
    tmp_vec.push(Item::Integer(value));
    let mut ret_vec = Vec::new();
    ret_vec.push(Item::ItemList(tmp_vec));
    ret_vec
}

fn part1(pairs: &Input) -> usize {
    let mut sum = 0;
    for i in 0..pairs.len() {
        if compare_lists(&pairs[i].0, &pairs[i].1) == Ordering::Less {
            sum += (i + 1);
        }
    }

    sum
}

fn part2(pairs: &Input) -> usize {
    let mut items = Vec::new();
    for p in pairs {
        items.push(p.0.clone());
        items.push(p.1.clone());
    }
    
    let marker2 = create_marker(2);
    items.push(marker2.clone());
    let marker6 = create_marker(6);
    items.push(marker6.clone());

    items.sort_by(compare_lists);

    let mut i2 = 0;
    let mut i6 = 0;
    for i in 0..items.len() {
        if compare_lists(&marker2, &items[i]) == Ordering::Equal {
            i2 = i + 1;
        }
        if compare_lists(&marker6, &items[i]) == Ordering::Equal {
            i6 = i + 1;
        }
    }

    i2 * i6
}

pub fn main() {
    let inst = parse_input("./input/day13/input.txt");
    
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
    fn day13_test1() {
        let inst = parse_input("./input/day13/test.txt");
        assert_eq!(part1(&inst), 13);
	}

    #[test]
    fn day13_test2() {
        let inst = parse_input("./input/day13/test.txt");
        assert_eq!(part2(&inst), 140);
	}
}
