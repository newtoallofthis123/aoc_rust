#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Entity {
    Space,
    Galaxy,
}

fn main() {
    let input = include_str!("temp")
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    '.' => Entity::Space,
                    '#' => Entity::Galaxy,
                    _ => panic!("Unknown entity"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let grid = return_full_input(input);

    // find galaxy positions
    let galaxy_positions = grid
        .iter()
        .enumerate()
        .flat_map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(_j, col)| col == &&Entity::Galaxy)
                .map(|(j, _col)| (i, j))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", galaxy_positions);

    let mut distances = Vec::new();

    for (i, j) in galaxy_positions.clone() {
        let mut distance = Vec::new();
        for (k, l) in galaxy_positions.iter() {
            distance.push((i as i32 - *k as i32).abs() + (j as i32 - *l as i32).abs());
        }
        distances.push(distance);
    }

    println!("{:?}", distances);

    let mut total = 0;

    distances.iter().for_each(|d| {
        total += d.iter().sum::<i32>();
    });

    println!("{}", total/2);
}

fn return_full_input(input:Vec<Vec<Entity>>) -> Vec<Vec<Entity>>{
    let mut input = input;

    let empty_rows = input
        .iter()
        .enumerate()
        .filter(|(_i, row)| {
            row.iter()
                .enumerate()
                .all(|(_j, col)| col == &Entity::Space)
        })
        .map(|(i, _row)| i)
        .collect::<Vec<_>>();

    let mut empty_cols = Vec::new();

    for j in 0..input[0].len() {
        let mut is_empty = true;
        (0..input.len()).for_each(|i| {
            if input[i][j] != Entity::Space {
                is_empty = false;
            }
        });
        if is_empty {
            empty_cols.push(j);
        }
    }

    let empty_row = input[empty_rows[0]].clone();

    let mut x = 0;
    empty_rows.iter().for_each(|i| {
        input.insert(*i + x, empty_row.clone());
        x += 1;
    });

    x = 0;
    empty_cols.iter().for_each(|j| {
        input.iter_mut().for_each(|row| {
            row.insert(*j + x, Entity::Space);
        });
        x += 1;
    });

    input
}
