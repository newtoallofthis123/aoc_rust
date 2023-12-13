// Thanks for the reference https://github.com/dreamlax/aoc2023/blob/main/day13/src/main.rs

fn main() {
    let input = include_str!("input")
        .split("\n\n")
        .map(|x| x.split('\n').map(|x| x.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let total = input.iter().map(|x| find_answer(x.clone())).sum::<usize>();

    println!("Total: {}", total);
}

fn get_diff(to: &str, from: &str) -> usize {
    to.chars()
        .zip(from.chars())
        .fold(0, |acc, (c1, c2)| match c1 == c2 {
            true => acc,
            false => acc + 1,
        })
}

fn find_answer(pattern: Vec<String>) -> usize {
    if let Some(last) = calc_rows(pattern.clone(), 1) {
        return last * 100;
    }

    let transposed = transpose(pattern);
    calc_rows(transposed, 1).expect("No mirror found in row or column")
}

fn calc_rows(pattern: Vec<String>, diff: usize) -> Option<usize> {
    pattern
        .windows(2)
        .enumerate()
        .filter_map(|(idx, rows)| {
            let d = get_diff(&rows[0], &rows[1]);
            if d <= diff {
                Some((d, idx))
            } else {
                None
            }
        })
        .find_map(|(existing_diff, idx)| {
            let diff_remaining = diff - existing_diff;
            if check_mirror(pattern.clone(), idx) == diff_remaining {
                Some(idx + 1)
            } else {
                None
            }
        })
}

fn transpose(pattern: Vec<String>) -> Vec<String> {
    (0..pattern[0].len())
        .map(|i| {
            pattern
                .iter()
                .map(|row| row.chars().nth(i).unwrap())
                .collect()
        })
        .collect()
}

fn check_mirror(lines: Vec<String>, mirror_point: usize) -> usize {
    if mirror_point == 0 || mirror_point == lines.len() - 2 {
        return 0;
    }

    let rows_to_compare = (lines.len() - mirror_point - 2).min(mirror_point);
    let mut diff = 0;
    for idx in 1..=rows_to_compare {
        diff += get_diff(&lines[mirror_point - idx], &lines[mirror_point + idx + 1]);
    }

    diff
}
