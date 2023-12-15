#[derive(Debug, Clone)]
pub enum Proc {
    Add(String, u128),
    Remove(String),
}

fn main() {
    let sequence = include_str!("input")
        .split(',')
        .map(|thing| {
            let chars = thing.chars().collect::<Vec<_>>();
            let label = chars[0..(chars.len() - 2)].iter().collect::<String>();
            if chars.contains(&'=') {
                Proc::Add(label, chars[chars.len() - 1].to_digit(10).unwrap() as u128)
            } else if chars.contains(&'-') {
                Proc::Remove(chars[0..(chars.len() - 1)].iter().collect::<String>())
            } else {
                panic!("Invalid input");
            }
        })
        .collect::<Vec<_>>();
    
    let mut boxes: Vec<Vec<(String, u128)>> = vec![Vec::new(); 1000];
    for seq in sequence {
        match seq {
            Proc::Add(label, num) => {
                let hash = calc_hash(label.clone()) as usize;
                println!("{} {}->{}", label, num, hash);
                match boxes.get_mut(hash) {
                    Some(pair) => {
                        if pair.iter().any(|x| x.0 == label) {
                            pair.iter_mut().find(|x| x.0 == label).unwrap().1 = num;
                        } else {
                            pair.push((label.clone(), num));
                        }
                    }
                    None => {
                        boxes.insert(
                            calc_hash(label.clone()) as usize,
                            vec![(label.clone(), num)],
                        );
                    }
                }
            }
            Proc::Remove(label) => {
                let hash = calc_hash(label.clone()) as usize;
                if let Some(pair) = boxes.get_mut(hash) {
                    pair.retain(|x| x.0 != label);
                }
            }
        }
    }

    let mut total = 0;

    for (i, b) in boxes.iter().enumerate() {
        if !b.is_empty() {
            println!("{:?}", b);
            total += b
                .iter()
                .enumerate()
                .map(|(j, x)| (1 + (i as u128)) * (1 + (j as u128)) * x.1)
                .sum::<u128>();
        }
    }

    println!("Total: {}", total);
}

fn calc_hash(str: String) -> u128 {
    let mut curr_val: u128 = 0;

    for ch in str.chars() {
        curr_val += ch as u128;
        curr_val *= 17;
        curr_val %= 256;
    }

    curr_val
}
