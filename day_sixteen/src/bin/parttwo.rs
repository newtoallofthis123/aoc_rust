fn main() {
    let input = include_str!("input")
        .lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut max = 0;

    // going right
    for i in 0..input.len(){
        if max < get_tiles(&input, (i, 0, 1)){
            max = get_tiles(&input, (i, 0, 1));
        }
    }

    // going down
    for i in 0..input[0].len(){
        if max < get_tiles(&input, (0, i, 2)){
            max = get_tiles(&input, (0, i, 2));
        }
    }

    // going left
    for i in 0..input.len(){
        if max < get_tiles(&input, (i, input[0].len()-1, 3)){
            max = get_tiles(&input, (i, input[0].len()-1, 3));
        }
    }

    // going up
    for i in 0..input[0].len(){
        if max < get_tiles(&input, (input.len()-1, i, 0)){
            max = get_tiles(&input, (input.len()-1, i, 0));
        }
    }

    println!("{}", max);
}


fn next(row: usize, col: usize, direction: usize) -> (usize, usize, usize){
    let (x_direction, y_directions) = [(-1,0),(0,1),(1,0),(0,-1)][direction];
    ((row as isize + x_direction) as _, (col as isize + y_directions) as _, direction)
}

fn get_tiles(grid: &Vec<Vec<char>>, start: (usize,usize,usize)) -> usize{
    let mut visited = vec![vec![[false; 4]; grid[0].len()]; grid.len()];
    let mut queue = vec![start];

    while !queue.is_empty(){
        let mut beams = Vec::with_capacity(queue.capacity());
        for(row, col, direction) in queue{
            if row >= grid.len() || col >= grid[0].len() {
                continue;
            }
            if visited[row][col][direction]{
                continue;
            }

            visited[row][col][direction] = true;
            match (grid[row][col], direction){
                ('.', _) => {
                    beams.push(next(row, col, direction))
                }
                ('/', _) => {
                    beams.push(next(row, col, [1, 0, 3, 2][direction]))
                }
                ('\\', _) => {
                    beams.push(next(row, col, [3, 2, 1, 0][direction]))
                }
                ('|', 0|2) => {
                    beams.push(next(row, col, direction))
                }
                ('-', 1|3) => {
                    beams.push(next(row, col, direction))
                }
                ('|', _) => {
                    beams.extend([next(row, col, 0), next(row, col, 2)])
                }
                ('-', _) => {
                    beams.extend([next(row, col, 1), next(row, col, 3)])
                }
                _ => unreachable!()
            }
        }
        queue = beams;
    }

    visited.iter().flatten().filter(|x| x.iter().any(|is| *is)).count()
}