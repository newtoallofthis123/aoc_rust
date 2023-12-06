fn main() {
    let input = include_str!("input")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let times = input[0]
        .split(' ')
        .filter(|x| is_numeric(x))
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let distances = input[1]
        .split(' ')
        .filter(|x| is_numeric(x))
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let pairs = times
        .iter()
        .zip(distances.iter())
        .map(|(x, y)| (x.to_owned(), y.to_owned()))
        .collect::<Vec<(i32, i32)>>();

    println!("{:?}", pairs);

    let mut total = 1;

    for pair in pairs {
        let winning = get_winning(pair);
        total *= winning.len() as i32;
    }

    println!("Total: {}", total);
}

fn is_numeric(num: &str) -> bool {
    num.parse::<i32>().is_ok()
}

fn get_winning(pair: (i32, i32)) -> Vec<i32> {
    let (time, record) = pair;
    let mut res = Vec::new();

    for i in 1..time {
        let distance = i * (time - i);
        println!("Distance: {}", distance);

        if distance > record {
            res.push(i);
        }
    }

    res
}

// 393120
