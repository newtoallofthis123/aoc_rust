#[derive(Debug, Clone, Copy, PartialEq)]
enum Trench {
    D,
    U,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    U,
    D,
    L,
    R,
}

#[derive(Debug, Clone, Copy)]
struct Entry {
    direction: Direction,
    distance: i32,
}

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|x| {
            let split = x.split_whitespace().collect::<Vec<&str>>();
            Entry {
                direction: match split[0] {
                    "U" => Direction::U,
                    "D" => Direction::D,
                    "L" => Direction::L,
                    "R" => Direction::R,
                    _ => panic!("Invalid direction"),
                },
                distance: split[1].parse::<i32>().unwrap(),
            }
        })
        .collect::<Vec<Entry>>();

    let mut grid = vec![vec![Trench::U; 1500]; 1500];
    let mut curr_pos = (0, 0);
    for entry in input.iter() {
        println!("{:?}", entry);
        match entry.direction {
            Direction::R => {
                for _ in 0..entry.distance {
                    curr_pos.1 += 1;
                    grid[curr_pos.0][curr_pos.1] = Trench::D;
                }
            }
            Direction::L => {
                for _ in 0..entry.distance {
                    if curr_pos.1 == 0 {
                        continue;
                    }
                    curr_pos.1 -= 1;
                    grid[curr_pos.0][curr_pos.1] = Trench::D;
                }
            }
            Direction::U => {
                for _ in 0..entry.distance {
                    if curr_pos.0 == 0 {
                        continue;
                    }
                    curr_pos.0 -= 1;
                    grid[curr_pos.0][curr_pos.1] = Trench::D;
                }
            }
            Direction::D => {
                for _ in 0..entry.distance {
                    curr_pos.0 += 1;
                    grid[curr_pos.0][curr_pos.1] = Trench::D;
                }
            }
        }
    }

    // pretty_print(&grid);

    let mut area = 0;
    for line in grid.clone() {
        let d_pos = line
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == Trench::D)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>();
        println!("{:?}", d_pos);
        if d_pos.is_empty(){
            continue;
        } else{
            area += d_pos.last().unwrap() - d_pos.first().unwrap() + 1;
        }
    }

    pretty_print(&grid);
    println!("Area: {}", area);
}

fn pretty_print(grid: &[Vec<Trench>]) {
    for row in grid.iter() {
        for col in row.iter() {
            match col {
                Trench::D => print!("#"),
                Trench::U => print!("."),
            }
        }
        println!();
    }
}
