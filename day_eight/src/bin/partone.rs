use std::collections::BTreeMap;

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let instructions = input[0].chars().collect::<Vec<char>>();
    let map_input = input[2..].to_vec();
    let mut map = BTreeMap::new();

    for line in map_input.iter() {
        let vals = line.split(' ').collect::<Vec<&str>>();
        let name = vals[0];

        let left = vals[2]
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<String>();
        let right = vals[3]
            .chars()
            .filter(|x| x.is_alphabetic())
            .collect::<String>();

        map.insert(name, (left, right));
    }

    let mut current = "AAA".to_string();
    let Some(count) =
        instructions
            .iter()
            .cycle()
            .enumerate()
            .find_map(|(i, direction)| -> Option<usize> {
                let neighbors = map.get(current.as_str()).expect("No neighbors").to_owned();
                let next = match *direction {
                    'L' => neighbors.0,
                    'R' => neighbors.1,
                    _ => panic!("wut?"),
                };
                if next == "ZZZ" {
                    Some(i + 1)
                } else {
                    current = next;
                    None
                }
            })
    else {
        panic!("No path found")
    };

    println!("Steps: {}", count);
}
