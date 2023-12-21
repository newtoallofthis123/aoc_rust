//! TODO
// Doesn't work 

use std::collections::HashMap;
use rayon::prelude::*;

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

    let mut full = Vec::new();
    seeds.chunks_exact(2).for_each(|range| {
        let (start, till) = (range[0], range[1]);   
        let temp_seeds = (start..start+till).collect::<Vec<i64>>();
        full.extend(temp_seeds);
    });

    let seeds = full;

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
        .par_iter()
        .map(|x| get_map_result(maps.get("seed-to-soil").unwrap().clone(), *x))
        .inspect(|x| println!("{}", x))
        .collect::<Vec<i64>>();
    let fertilizers = soils
        .par_iter()
        .map(|x| get_map_result(maps.get("soil-to-fertilizer").unwrap().clone(), *x))
        .inspect(|x| println!("{}", x))
        .collect::<Vec<i64>>();
    let water = fertilizers
        .par_iter()
        .map(|x| get_map_result(maps.get("fertilizer-to-water").unwrap().clone(), *x))
        .collect::<Vec<i64>>();

    let light = water
        .par_iter()
        .map(|x| get_map_result(maps.get("water-to-light").unwrap().clone(), *x))
        .inspect(|x| println!("{}", x))
        .collect::<Vec<i64>>();
    let temperature = light
        .par_iter()
        .map(|x| get_map_result(maps.get("light-to-temperature").unwrap().clone(), *x))
        .inspect(|x| println!("{}", x))
        .collect::<Vec<i64>>();
    let humidity = temperature
        .par_iter()
        .map(|x| get_map_result(maps.get("temperature-to-humidity").unwrap().clone(), *x))
        .inspect(|x| println!("{}", x))
        .collect::<Vec<i64>>();
    let location = humidity
        .par_iter()
        .map(|x| get_map_result(maps.get("humidity-to-location").unwrap().clone(), *x))
        .inspect(|x| println!("{}", x))
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
