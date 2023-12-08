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
        let name = vals[0].to_string();

        let left = vals[2]
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect::<String>();
        let right = vals[3]
            .chars()
            .filter(|x| x.is_alphanumeric())
            .collect::<String>();

        map.insert(name, (left, right));
    }

    // println!("{:?}", map);

    let curr_map = map
        .iter()
        .filter(|(k, _)| k.ends_with('A'))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<BTreeMap<String, (String, String)>>();

    println!("{:?}", curr_map);

    let res = curr_map.iter().map(|entry| {
        let mut curr = entry.0.to_owned();
        instructions.iter().cycle().enumerate().find_map(|(i, direction)| {
            let neighbors = map.get(curr.as_str()).expect("No neighbors").to_owned();
            let next = match *direction {
                'L' => {
                    neighbors.0
                }
                'R' => {
                    neighbors.1
                }
                _ => panic!("wut?")
            };

            if next.ends_with('Z') {
                Some(i+1)
            } else {
                curr = next;
                None
            }
        }).unwrap()
    }).collect::<Vec<usize>>();

    println!("{:?}", lcm(res.as_slice()));
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}