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
            continue;
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

fn compare_lists(left: &Vec<Item>, right: &Vec<Item>) -> Ordering {
    let mut count = 0;
    loop {
        if count == left.len() && count == right.len() {
            return Ordering::Equal;
        }
        if count == left.len() {
            return Ordering::Less;
        }
        if count == right.len() {
            return Ordering::Greater;
        }
        match &left[count] {
            Item::Integer(lv) => {
                match &right[count] {
                    Item::Integer(rv) => {
                        if lv < rv {
                            return Ordering::Less;
                        }
                        if lv > rv {
                            return Ordering::Greater;
                        }
                    }
                    Item::ItemList(rv) => {
                        let mut tmp_vec = Vec::new();
                        tmp_vec.push(Item::Integer(*lv));
                        let result = compare_lists(&tmp_vec, rv);
                        match result {
                            Ordering::Equal => {},
                            _ => { return result; }
                        }
                    }
                }
            }
            Item::ItemList(llist) => {
                match &right[count] {
                    Item::Integer(rv) => {
                        let mut tmp_vec = Vec::new();
                        tmp_vec.push(Item::Integer(*rv));
                        let result = compare_lists(llist, &tmp_vec);
                        match result {
                            Ordering::Equal => {},
                            _ => { return result; }
                        }
                    }
                    Item::ItemList(rlist) => {
                        let result = compare_lists(llist, rlist);
                        match result {
                            Ordering::Equal => {},
                            _ => { return result; }
                        }
                    }
                }
            }
        }
        count += 1;
    }
    Ordering::Equal;
}

fn part1(pairs: &Input) -> usize {
    let mut sum = 0;
    for i in 0..pairs.len() {
        let result = compare_lists(&pairs[i].0, &pairs[i].1);
        match result {
            Ordering::Less => {
                sum += (i + 1);
            },
            _ => {}
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
    
    let mut tmp_vec = Vec::new();
    tmp_vec.push(Item::Integer(2));
    let mut tmp_vec2 = Vec::new();
    tmp_vec2.push(Item::ItemList(tmp_vec));
    items.push(tmp_vec2.clone());

    let mut tmp_vec = Vec::new();
    tmp_vec.push(Item::Integer(6));
    let mut tmp_vec6 = Vec::new();
    tmp_vec6.push(Item::ItemList(tmp_vec));
    items.push(tmp_vec6.clone());

    items.sort_by(compare_lists);

    let mut i2 = 0;
    let mut i6 = 0;
    for i in 0..items.len() {
        match compare_lists(&tmp_vec2, &items[i]) {
            Ordering::Equal => {
                i2 = i + 1;
            }
            _ => {}
        }
        match compare_lists(&tmp_vec6, &items[i]) {
            Ordering::Equal => {
                i6 = i + 1;
            }
            _ => {}
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
