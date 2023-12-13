#[derive(Debug, Clone, Copy, PartialEq)]
enum Machine {
    Broken,
    Working,
    Unknown,
}

fn main() {
    let input = include_str!("temp")
        .lines()
        .map(|x| x.split(' ').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let records = input
        .iter()
        .map(|x| {
            x[1].split(',')
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();
    let input = input
        .iter()
        .map(|x| {
            x[0].chars()
                .map(|y| match y {
                    '?' => Machine::Unknown,
                    '#' => Machine::Broken,
                    '.' => Machine::Working,
                    _ => panic!("Unknown machine type"),
                })
                .collect::<Vec<Machine>>()
        })
        .collect::<Vec<Vec<Machine>>>();

    let map = input
        .iter()
        .zip(records.iter())
        .map(|(x, y)| (x.clone(), y.clone()))
        .collect::<Vec<(Vec<Machine>, Vec<i32>)>>()
        .to_vec();

    // println!("{:?}", map);

    let test = map[2].clone();
    println!("{:?}", test);

    arrange_broken(test.0, test.1);
}

fn arrange_broken(machines: Vec<Machine>, records: Vec<i32>) {
    println!("{:?}", machines);  
}