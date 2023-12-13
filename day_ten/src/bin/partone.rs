const UP: [char; 3] = ['|', 'F', '7'];
const DOWN: [char; 3] = ['|', 'L', 'J'];
const LEFT: [char; 3] = ['-', 'F', 'L'];
const RIGHT: [char; 3] = ['-', '7', 'J'];

fn main() {
    let grid = include_str!("temp")
        .lines()
        .map(|x| x.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // pos of 'S'
    let (x, y) = grid
        .iter()
        .enumerate()
        .find_map(|(y, row)| {
            row.iter()
                .enumerate()
                .find_map(|(x, dig)| if *dig == 'S' { Some((x, y)) } else { None })
        })
        .unwrap();

    println!("x: {}, y: {}", x, y);

    // for (y, row) in grid.iter().enumerate() {
    //     for (x, dig) in row.iter().enumerate() {
    //         ;
    //     }
    // }

    println!("options: {:?}", get_neighbors(&grid, (2, 1)))
}

fn get_neighbors(grid: &Vec<Vec<char>>, (x, y): (i32, i32)) -> Vec<(usize, usize)> {
    let options = [(x, y + 1), (x, y - 1), (x + 1, y), (x - 1, y)]
        .iter()
        .filter(|(x, y)| does_exist(grid, (*x, *y)))
        .map(|(x, y)| (*x as usize, *y as usize))
        .collect::<Vec<(usize, usize)>>()
        .iter()
        .filter(|(x, y)| {is_pipe(grid[*x][*y])})
        .cloned()
        .collect::<Vec<(usize, usize)>>();

    let pipe = grid[x as usize][y as usize];

    println!("pipe: {}", pipe);
    println!("X: {}, Y: {}", x-1, y);

    match pipe {
        'S' => {
            options.iter()
                .filter(|(x, y)| {
                    if does_exist(grid, (*x as i32, *y as i32 + 1)) && is_pipe(grid[*x][*y + 1]) && RIGHT.contains(&grid[*x][*y + 1]) {
                        return true;
                    }

                    if does_exist(grid, (*x as i32, *y as i32 - 1)) && is_pipe(grid[*x][*y - 1]) && LEFT.contains(&grid[*x][*y - 1]) {
                        return true;
                    }

                    if does_exist(grid, (*x as i32 + 1, *y as i32)) && is_pipe(grid[*x + 1][*y]) && DOWN.contains(&grid[*x + 1][*y]) {
                        return true;
                    }

                    if does_exist(grid, (*x as i32 - 1, *y as i32)) && is_pipe(grid[*x - 1][*y]) && UP.contains(&grid[*x - 1][*y]) {
                        return true;
                    }

                    false
                })
                .cloned()
                .collect::<Vec<(usize, usize)>>()
        }

        'J' => {
            let mut res = Vec::new();

            if does_exist(grid, (x - 1, y)) && is_pipe(grid[x as usize - 1][y as usize]) && UP.contains(&grid[x as usize - 1][y as usize]) {
                res.push((x as usize - 1, y as usize));
            }

            if does_exist(grid, (x, y - 1)) && is_pipe(grid[x as usize][y as usize - 1]) && LEFT.contains(&grid[x as usize][y as usize - 1]) {
                res.push((x as usize, y as usize - 1));
            }

            res
        }

        'L' => {
            let mut res = Vec::new();

            if does_exist(grid, (x + 1, y)) && is_pipe(grid[x as usize + 1][y as usize]) && DOWN.contains(&grid[x as usize + 1][y as usize]) {
                res.push((x as usize + 1, y as usize));
            }

            if does_exist(grid, (x, y + 1)) && is_pipe(grid[x as usize][y as usize + 1]) && RIGHT.contains(&grid[x as usize][y as usize + 1]) {
                res.push((x as usize, y as usize + 1));
            }

            res
        }

        'F' => {
            let mut res = Vec::new();

            if does_exist(grid, (x + 1, y)) && is_pipe(grid[x as usize + 1][y as usize]) && DOWN.contains(&grid[x as usize + 1][y as usize]) {
                res.push((x as usize + 1, y as usize));
            }

            if does_exist(grid, (x, y + 1)) && is_pipe(grid[x as usize][y as usize + 1]) && RIGHT.contains(&grid[x as usize][y as usize + 1]) {
                res.push((x as usize, y as usize + 1));
            }

            res
        }

        '7' => {
            let mut res = Vec::new();

            if does_exist(grid, (x, y - 1)) && is_pipe(grid[x as usize][y as usize - 1]) && LEFT.contains(&grid[x as usize][y as usize - 1]) {
                res.push((x as usize, y as usize - 1));
            }

            if does_exist(grid, (x - 1, y)) && is_pipe(grid[x as usize - 1][y as usize]) && UP.contains(&grid[x as usize - 1][y as usize]) {
                res.push((x as usize - 1, y as usize));
            }

            res
        }

        '|' => {
            let mut res = Vec::new();

            if does_exist(grid, (x + 1, y)) && is_pipe(grid[x as usize + 1][y as usize]) && DOWN.contains(&grid[x as usize + 1][y as usize]) {
                res.push((x as usize + 1, y as usize));
            }

            if does_exist(grid, (x - 1, y)) && is_pipe(grid[x as usize - 1][y as usize]) && UP.contains(&grid[x as usize - 1][y as usize]) {
                res.push((x as usize - 1, y as usize));
            }

            res
        }

        '-' => {
            let mut res = Vec::new();

            if does_exist(grid, (x, y + 1)) && is_pipe(grid[x as usize][y as usize + 1]) && RIGHT.contains(&grid[x as usize][y as usize + 1]) {
                res.push((x as usize, y as usize + 1));
            }

            if does_exist(grid, (x, y - 1)) && is_pipe(grid[x as usize][y as usize - 1]) && LEFT.contains(&grid[x as usize][y as usize - 1]) {
                res.push((x as usize, y as usize - 1));
            }

            res
        }

        _ => {
            options
        }
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