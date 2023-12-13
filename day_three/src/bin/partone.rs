use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entry {
    Number(i32),
    Symbol,
    Stopper,
}

fn main() {
    let input = include_str!("temp").lines().collect::<Vec<_>>();
    let mut grid = BTreeMap::new();

    for (line_num, line) in input.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            let entry = match c {
                '.' => Entry::Stopper,
                '0'..='9' => Entry::Number(c.to_digit(10).unwrap() as i32),
                _ => Entry::Symbol,
            };

            grid.insert((line_num as i32, i as i32), entry);
        }
    }

    let mut total = 0;

    for line in 0..input.len() {
        let line_nums = get_nums_in_line(&grid, line as i32);

        if line_nums.is_empty() {
            continue;
        }

        for (num, pos) in line_nums {
            let indices = get_possible_indexes(pos.clone());

            if check_num_sides(&grid, line as i32, pos[0]) || check_num_sides(&grid, line as i32, pos[pos.len() - 1]) {
                total += num;
                println!("{} is a possible solution", num);
            }

            for index in indices {
                if line == 0 || line == input.len() - 1 {
                    if line == 0 && check_position(&grid, line as i32 + 1, index) || line == input.len() - 1 && check_position(&grid, line as i32 - 1, index){
                        total += num;
                        println!("{} is a possible solution", num);
                    }
                } else if check_position(&grid, line as i32 - 1, index) || check_position(&grid, line as i32 + 1, index) {
                    total += num;
                    println!("{} is a possible solution", num);
                }
            }
        }
    }

    println!("total: {}", total);
}

fn check_num_sides(grid: &BTreeMap<(i32, i32), Entry>, x: i32, y: i32) -> bool {
    check_position(grid, x, y - 1) || check_position(grid, x, y + 1)
}

fn check_position(grid: &BTreeMap<(i32, i32), Entry>, x: i32, y: i32) -> bool {
    grid.get(&(x, y)).unwrap_or(&Entry::Stopper) == &Entry::Symbol
}

fn get_nums_in_line(grid: &BTreeMap<(i32, i32), Entry>, x: i32) -> Vec<(i32, Vec<i32>)> {
    let mut nums = Vec::new();
    let mut temp = Vec::new();

    for y in 0..grid.len() {
        let entry = grid.get(&(x, y as i32)).unwrap_or(&Entry::Stopper);

        match entry {
            Entry::Number(n) => temp.push((*n, y as i32)),
            Entry::Symbol | Entry::Stopper => {
                if !temp.is_empty() {
                    let num = temp.iter().fold(0, |acc, x| acc * 10 + x.0);
                    let indexes = temp.iter().map(|x| x.1).collect::<Vec<_>>();
                    nums.push((num, indexes));
                    temp.clear();
                }
            }
        }
    }
    
    nums
}

fn get_possible_indexes(pos: Vec<i32>) -> Vec<i32> {
    let first = pos.first().unwrap();
    let last = pos.last().unwrap();

    let mut indexes = Vec::new();

    if first > &0 {
        indexes.push(first - 1);
    }
    indexes.push(*first);

    indexes.extend(pos.iter().skip(1));

    indexes.push(last + 1);

    indexes
}

// 540025