/// The solution is correct, but it is not exactly elegant
/// The Problem of with handling very very large floats and numbers
/// is that mathematical formulae used are right, but you need
/// have a mastery of the floating point numbers to get the exact
/// solution.
/// In this case, I used quadratic formula to get the solution
/// So, for smaller numbers, in the i32 range, the solution is correct
/// however, when we delve into the i64 range, the solution is not exact
/// and might be off by one or two numbers. Hence, I needed to implement a
/// while loop check before returning the min and max numbers.
/// This check slows the efficiency of the program just a bit, but it is
/// still fast enough to get the correct answer in a few milliseconds.

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    
    let times = input[0]
        .chars()
        .filter(|x| x.is_numeric())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();
    let distances = input[1]
        .chars()
        .filter(|x| x.is_numeric())
        .collect::<String>()
        .parse::<i64>()
        .unwrap();

    let pair = (times, distances);

    println!("{:?}", pair);
    println!("{:?}", get_winning(pair));
}

fn get_winning(pair: (i64, i64)) -> i64 {
    let (time, record) = pair;

    // equation is -i^2 + time*i - record = 0
    // The i is the speed
    
    let diff = (time.pow(2) as f32 - (4.0 * record as f32)).sqrt();

    let res = [(time as f32 + diff) / 2.0, (time as f32 - diff) / 2.0];
    let mut min_number = res
        .iter()
        .min_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
        .to_owned()
        .ceil() as i64;

    let mut max_number = res
        .iter()
        .max_by(|x, y| x.partial_cmp(y).unwrap())
        .unwrap()
        .to_owned()
        .floor() as i64;

    while min_number * (time - min_number) <= record {
        min_number += 1;
    }
    while max_number * (time - max_number) < record {
        max_number -= 1;
    }

    println!("{:?}", (min_number, max_number));

    max_number - min_number + 1
}

// 36872656
