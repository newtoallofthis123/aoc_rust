//! TODO
// Doesn't work 

use std::collections::HashMap;

fn main() {
    let input = include_str!("temp")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    // let seeds = input[0]
    //     .split(':')
    //     .last()
    //     .unwrap()
    //     .strip_prefix(' ')
    //     .unwrap()
    //     .split(' ')
    //     .map(|x| x.parse::<i64>().unwrap())
    //     .collect::<Vec<i64>>();

    let seeds = (60, 9);

    let input_maps = input[1..]
        .iter()
        .map(|x| x.split('\n').collect::<Vec<&str>>())
        .collect::<Vec<Vec<&str>>>();

    let mut maps = HashMap::new();

    input_maps.iter().for_each(|map| {
        let map_name = map[0].strip_suffix(" map:").unwrap();
        let mut map_seeds = Vec::new();

        for entry in map[1..].iter() {
            map_seeds.push(
                entry
                    .split(' ')
                    .map(|x| x.parse::<i64>().unwrap())
                    .collect::<Vec<i64>>(),
            );
        }

        maps.insert(map_name, map_seeds);
    });

    println!("{:?}", get_range(maps.get("seed-to-soil").unwrap().clone(), seeds));
}

fn get_range(map: Vec<Vec<i64>>, range: (i64, i64)) -> (i64, i64) {
    println!("{:?}", (&map, &range));
    let mut min = 0;
    let mut min_entry = Vec::new();

    let (start, till) = range;

    for entry in map{
        let diff = entry[0] - entry[1];
        if diff < min && entry[1] < start+till{
            min = diff;
            min_entry = entry.clone();
        } 
        // else if entry[1] > start+till {
        //     min = diff;
        //     min_entry = entry.clone();
        // }
    }

    println!("{:?}", (&min, &min_entry));

    (0, 0)
}
