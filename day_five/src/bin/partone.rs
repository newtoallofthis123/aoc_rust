// 662197086

use std::collections::HashMap;

fn main() {
    let input = include_str!("input")
        .split("\n\n")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let seeds = input[0]
        .split(':')
        .last()
        .unwrap()
        .strip_prefix(' ')
        .unwrap()
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    println!("{:?}", seeds);

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

    let soils = seeds
        .iter()
        .map(|x| get_map_result(maps.get("seed-to-soil").unwrap().clone(), *x))
        .collect::<Vec<i64>>();
    let fertilizers = soils
        .iter()
        .map(|x| get_map_result(maps.get("soil-to-fertilizer").unwrap().clone(), *x))
        .collect::<Vec<i64>>();
    let water = fertilizers
        .iter()
        .map(|x| get_map_result(maps.get("fertilizer-to-water").unwrap().clone(), *x))
        .collect::<Vec<i64>>();

    let light = water
        .iter()
        .map(|x| get_map_result(maps.get("water-to-light").unwrap().clone(), *x))
        .collect::<Vec<i64>>();
    let temperature = light
        .iter()
        .map(|x| get_map_result(maps.get("light-to-temperature").unwrap().clone(), *x))
        .collect::<Vec<i64>>();
    let humidity = temperature
        .iter()
        .map(|x| get_map_result(maps.get("temperature-to-humidity").unwrap().clone(), *x))
        .collect::<Vec<i64>>();
    let location = humidity
        .iter()
        .map(|x| get_map_result(maps.get("humidity-to-location").unwrap().clone(), *x))
        .collect::<Vec<i64>>();

    println!("{:?}", location.iter().min().unwrap());
}

fn get_map_result(arr: Vec<Vec<i64>>, source: i64) -> i64 {
    let mut res = 0;

    for entry in arr {
        let diff = entry[0] - entry[1];
        if source >= entry[1] && source <= entry[1] + entry[2] {
            res = source + diff;
            if res > 0 {
                break;
            }
        }
    }

    match res {
        0 => source,
        _ => res,
    }
}
