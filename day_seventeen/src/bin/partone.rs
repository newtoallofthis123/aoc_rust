/// Used this https://github.dev/AxlLind/AdventOfCode2023/tree/main
/// Respect @AxlLind 🫡 

use std::collections::{BinaryHeap, HashMap};

fn main() {
    let input = include_str!("input")
        .lines()
        .map(str::as_bytes)
        .collect::<Vec<_>>();

    println!("{:?}", input);

    // The distances are stored as a tuple of (row, col, direction).
    let mut distances = HashMap::new();
    // Binary Heap is a sorted queue that can be used as a priority queue.
    // Elements are popped from the heap in descending order.
    // The BinaryHeap is backed by a Vec, and in general, has a O(n) lookup time.
    let mut queue = BinaryHeap::from_iter(vec![(0, (0, 0, (0, 0)))]);
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((cost, (row, col, direction))) = queue.pop() {
        if row == input.len() - 1 && col == input[0].len() - 1 {
            println!("Cost: {}", -cost);
            break;
        }

        if distances
            .get(&(row, col, direction))
            .is_some_and(|&c| -cost < c)
        {
            continue;
        }

        for (dr, dc) in directions.iter() {
            // We do not want to go back the way we came.
            // So, if the direction is the opposite of the current direction, we skip it.
            // Also, if the direction is the same as the current direction, we skip it.
            // This is because we can only change direction at a corner.
            if direction == (*dr, *dc) || direction == (-dr, -dc) {
                continue;
            }

            let mut next_cost = -cost;
            for distance in 1..=3 {
                let next_row = (row as isize + dr * distance) as usize;
                let next_col = (col as isize + dc * distance) as usize;

                if next_row >= input.len() || next_col >= input[0].len() {
                    continue;
                }

                next_cost += (input[next_row][next_col] - b'0') as i64;
                let key = (next_row, next_col, (*dr, *dc));
                if 1 <= distance && next_cost < *distances.get(&key).unwrap_or(&i64::MAX) {
                    distances.insert(key, next_cost);
                    queue.push((-next_cost, key));
                }
            }
        }
    }
}
