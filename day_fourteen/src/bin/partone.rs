/// My initial approach was to use bubble thing like in the bubble_approach.rs
/// But I found a solution by https://github.com/tymscar/Advent-Of-Code/blob/master/2023/rust/src/day14
/// that is just 🤌
/// So I implemented it here with some modifications
/// So, thanks @tymscar

fn main() {
    let lines = include_str!("temp")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut above = vec![0; lines[0].len()];
    println!("{:?}", above);

    let mut load = 0;
    for (vertical, line) in lines.iter().enumerate() {
        for (hor, c) in line.iter().enumerate() {
            match *c {
                '.' => {
                    above[hor] += 1;
                }
                '#' => {
                    above[hor] = 0;
                }
                'O' => {
                    load += lines.len() - (vertical - above[hor]);
                    println!("{}", load);
                }
                _ => panic!("Invalid rock type {}", *c),
            }
        }
    }

    println!("{}", load);
}
