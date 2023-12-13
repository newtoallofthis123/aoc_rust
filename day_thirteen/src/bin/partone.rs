fn main() {
    let input = include_str!("input")
        .split("\n\n")
        .map(|x| x.split('\n').map(|x| x.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let total = input.iter().map(|x| find_answer(x.clone())).sum::<usize>();

    println!("Total: {}", total);
}

fn find_answer(pattern: Vec<String>) -> usize {
    if let Some((_, last)) = calc_rows(pattern.clone()) {
        return last * 100;
    }

    let transposed = transpose(pattern);
    calc_rows(transposed)
        .expect("No mirror found in row or column")
        .1
}

fn calc_rows(pattern: Vec<String>) -> Option<(usize, usize)> {
    let mirror_images: Vec<(usize, usize)> = pattern
        .windows(2)
        .enumerate()
        .filter_map(|(i, pair)| {
            if pair[0] == pair[1] {
                Some((i, i + 1))
            } else {
                None
            }
        })
        .collect();

    let mut all_pairs = Vec::new();
    mirror_images.iter().for_each(|(first, last)| {
        let mut pairs = Vec::new();
        let mut f = *first;
        let mut l = *last;
        while l < pattern.len() {
            pairs.push((f, l));
            if f == 0 {
                break;
            }
            f -= 1;
            l += 1;
        }
        all_pairs.push(pairs);
    });

    let valid_pair = all_pairs.iter().find(|pairs| {
        pairs
            .iter()
            .all(|(first, last)| pattern[*first] == pattern[*last])
    });

    println!("Pairs: {:?}", valid_pair);
    println!("Mirror Images: {:?}", mirror_images);

    match valid_pair {
        Some(pairs) => {
            let (first, last) = pairs[0];
            Some((first, last))
        }
        None => None,
    }
}

fn transpose(pattern: Vec<String>) -> Vec<String> {
    let transposed_len = pattern[0].len();

    pattern.iter().fold(
        vec![String::with_capacity(pattern.len()); transposed_len],
        |mut acc, line| {
            line.chars()
                .enumerate()
                .for_each(|(idx, c)| acc[idx].push(c));
            acc
        },
    )
}
