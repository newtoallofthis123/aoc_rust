use std::vec;

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // pos of 'S'
    let (x, y) = input
        .iter()
        .enumerate()
        .filter(|(_, line)| line.contains(&'S'))
        .map(|(x, line)| (x, line.iter().position(|&ch| ch == 'S').unwrap()))
        .collect::<Vec<(usize, usize)>>()[0];

    println!("x: {}, y: {}", x, y);

    let mut queue = vec![(x, y)];
    let mut visited = vec![(x, y)];
    let mut steps = 0;

    while !queue.is_empty() {
        let mut new_queue = vec![];

        for (x, y) in queue {
            let neighbors = get_neighbors(&input, (x as i32, y as i32)).unwrap();

            for (x, y) in neighbors {
                if !visited.contains(&(x, y)) {
                    visited.push((x, y));
                    new_queue.push((x, y));
                }
            }
        }

        queue = new_queue;
        steps += 1;
    }

    println!("steps: {}", steps-1);
}

fn get_neighbors(map: &Vec<Vec<char>>, (x, y): (i32, i32)) -> Option<Vec<(usize, usize)>> {
    if x >= map.len() as i32 || y >= map[0].len() as i32 {
        return None;
    }

    let pipe = map[x as usize][y as usize];

    let options = [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
        .iter()
        .filter(|(x, y)| does_exist(map, (*x, *y)))
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect::<Vec<(usize, usize)>>();

    println!("pipe: {}", pipe);
    println!("options: {:?}", options);

    match pipe {
        'S' => {
            let mut res = Vec::new();

            if does_exist(map, (x, y+1)) && is_pipe(map[x as usize][y as usize+1]) && !['|', 'L', 'F'].contains(&map[x as usize][y as usize+1]) {
                res.push((x as usize, y as usize+1));
            }

            if does_exist(map, (x, y-1)) && is_pipe(map[x as usize][y as usize-1]) && !['|', '7', 'J'].contains(&map[x as usize][y as usize-1]) {
                res.push((x as usize, y as usize-1));
            }

            if does_exist(map, (x+1, y)) && is_pipe(map[x as usize + 1][y as usize]) && !['7', '-', 'F'].contains(&map[x as usize + 1][y as usize]) {
                res.push((x as usize + 1, y as usize));
            }

            if does_exist(map, (x-1, y)) && is_pipe(map[x as usize - 1][y as usize]) && !['L', '-', 'J'].contains(&map[x as usize - 1][y as usize]) {
                res.push((x as usize - 1, y as usize));
            }

            Some(res)
        }
        ,
        'J' => {
            let mut res = Vec::new();
            if does_exist(map, (x-1, y)) && is_pipe(map[x as usize - 1][y as usize]) && !['L', '-', 'J'].contains(&map[x as usize - 1][y as usize]) {
                res.push((x as usize - 1, y as usize));
            }
            if does_exist(map, (x, y-1)) && is_pipe(map[x as usize][y as usize-1]) && !['F', '7', 'J'].contains(&map[x as usize][y as usize-1]) {
                res.push((x as usize, y as usize-1));
            }

            Some(res)
        }
        'F' => {
            let mut res = Vec::new();
            if does_exist(map, (x+1, y)) && is_pipe(map[x as usize + 1][y as usize]) && !['7', '-', 'F'].contains(&map[x as usize + 1][y as usize]) {
                res.push((x as usize + 1, y as usize));
            }
            if does_exist(map, (x, y+1)) && is_pipe(map[x as usize][y as usize+1]) && !['|', 'L', 'F'].contains(&map[x as usize][y as usize+1]) {
                res.push((x as usize, y as usize+1));
            }

            Some(res)
        }
        'L' => {
            let mut res = Vec::new();
            if does_exist(map, (x, y+1)) && is_pipe(map[x as usize][y as usize+1]) && !['|', 'L', 'F'].contains(&map[x as usize][y as usize+1]) {
                res.push((x as usize, y as usize+1));
            }
            if does_exist(map, (x-1, y)) && is_pipe(map[x as usize - 1][y as usize]) && !['L', '-', 'J'].contains(&map[x as usize - 1][y as usize]) {
                res.push((x as usize - 1, y as usize));
            }

            Some(res)
        }
        '7' => {
            let mut res = Vec::new();
            if does_exist(map, (x, y-1)) && is_pipe(map[x as usize][y as usize-1]) && !['|', '7', 'J'].contains(&map[x as usize][y as usize-1]) {
                res.push((x as usize, y as usize-1));
            }
            if does_exist(map, (x+1, y)) && is_pipe(map[x as usize + 1][y as usize]) && !['7', '-', 'F'].contains(&map[x as usize + 1][y as usize]) {
                res.push((x as usize + 1, y as usize));
            }

            Some(res)
        }
        '-' => {
            let mut res = Vec::new();
            if does_exist(map, (x, y+1)) && is_pipe(map[x as usize][y as usize+1]) && !['|', 'L', 'F'].contains(&map[x as usize][y as usize+1]) {
                res.push((x as usize, y as usize+1));
            }
            if does_exist(map, (x, y-1)) && is_pipe(map[x as usize][y as usize-1]) && !['|', '7', 'J'].contains(&map[x as usize][y as usize-1]) {
                res.push((x as usize, y as usize-1));
            }

            Some(res)
        }
        '|' => {
            let mut res = Vec::new();
            if does_exist(map, (x-1, y)) && is_pipe(map[x as usize - 1][y as usize]) && !['L', '-', 'J'].contains(&map[x as usize - 1][y as usize]) {
                res.push((x as usize - 1, y as usize));
            }
            if does_exist(map, (x+1, y)) && is_pipe(map[x as usize + 1][y as usize]) && !['7', '-', 'F'].contains(&map[x as usize + 1][y as usize]) {
                res.push((x as usize + 1, y as usize));
            }

            Some(res)
        }
        _ => None,
    }
}

fn does_exist(map: &Vec<Vec<char>>, (x, y): (i32, i32)) -> bool {
    if x >= map.len() as i32 || y >= map[0].len() as i32 {
        return false;
    }

    if x < 0 || y < 0 {
        return false;
    }

    let pipe = map
        .get(x as usize)
        .unwrap_or(&vec![])
        .get(y as usize)
        .unwrap_or(&' ')
        .to_owned();

    !matches!(pipe, ' ')
}

fn is_pipe(pipe: char) -> bool {
    matches!(pipe, '|' | '-' | 'F' | 'L' | '7' | 'J' | 'S')
}