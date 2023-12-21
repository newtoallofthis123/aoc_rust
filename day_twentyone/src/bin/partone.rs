use std::collections::{HashMap, VecDeque};

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
const MAX_STEPS: usize = 64;

fn main() {
    let mut starting_pos = (0usize, 0usize);

    let grid: Vec<Vec<char>> = include_str!("input")
        .lines()
        .enumerate()
        .map(|(i, l)| {
            let row: Vec<_> = l
                .chars()
                .enumerate()
                .map(|(j, c)| {
                    if c == 'S' {
                        starting_pos = (i, j);
                    }
                    c
                })
                .collect();
            row
        })
        .collect();

    println!("{:?}", grid);
    println!("{:?}", starting_pos);

    let mut queue = VecDeque::new();
    let mut distances = HashMap::new();
    distances.insert(starting_pos, 0);
    queue.push_back((starting_pos, MAX_STEPS));

    while let Some(((i, j), steps)) = queue.pop_front() {
        for (di, dj) in &DIRECTIONS {
            let (ni, nj) = (i as isize + di, j as isize + dj);
            let (ni, nj) = (ni as usize, nj as usize);

            if ni < grid.len() && nj < grid[ni].len() && grid[ni][nj] != '#' {
                let next_pos = (ni, nj);
                if !distances.contains_key(&next_pos) && steps > 0 {
                    distances.insert(next_pos, steps - 1);
                    queue.push_back((next_pos, steps - 1));
                }
            }
        }
    }

    println!("{:?}", distances);

    let res = distances
        .values()
        .filter(|&&d| (d + MAX_STEPS % 2) % 2 == 0)
        .count();
    println!("{}", res);
}
