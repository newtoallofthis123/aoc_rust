/// I basically implemented the solution from the reddit thread
/// So I want to write these notes so that I can actually understand and learn from this
/// instead of just copy pasting the solution
/// So here, we use 0, 1, 2, 3 to represent the direction
/// 0 is up, 1 is right, 2 is down, 3 is left
/// So, to calculate the next position, we just add the direction to the row and column
/// to the direction which is the index of the array for direction tuples (x, y) as (0, 1) for right
/// and so on.
/// Now, we perform simple BFS, we start from the start position, and then we check the next position
/// and then we check the next position and so on.
/// For every queue iteration however, we use a special combination of the [north, east, south, west] 
/// directions to determine the next direction.
/// So, to move from north to east, we use [1, 0, 3, 2] and to move from north to west, we use [3, 2, 1, 0]
/// This is because we are using the direction as an index to the array, so we need to use the correct
/// index to get the correct direction.
/// Now, we check the current position and the direction, and then we push the next position into the queue
/// and then we continue.
/// A new queue is constructed from the beams, and then we continue.
/// We also have been tracking everything using the visited array, so we don't visit the same position
/// twice.
/// We also check if the position is out of bounds, and if it is, we just continue.
/// Then, we see the count of true values in the visited array, and that is the answer.

fn main() {
    let input = include_str!("input")
        .lines().map(|x| x.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    println!("{:?}", get_tiles(&input, (0, 0, 1)));
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