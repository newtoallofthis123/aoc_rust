fn main() {
    let input = include_str!("input")
        .lines()
        .map(|x| {
            x.split(' ')
                .map(|y| y.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let sum = input.iter().map(|x| find_val(x.clone())).sum::<i32>();

    println!("Sum: {}", sum);
}

fn find_val(line: Vec<i32>) -> i32 {
    let mut diffs: Vec<Vec<i32>> = Vec::new(); 
    let mut nums = line.clone();

    loop {
        let diff: Vec<i32> = nums.windows(2).map(|pair| pair[1] - pair[0]).collect();
        if diff.iter().all(|x| *x ==0 ){
            break;
        }
        nums = diff.clone();
        diffs.push(diff);
    }

    println!("Diffs: {:?}", diffs);

    let mut next = 0;
    diffs.iter().rev().for_each(|x| {
        next = x.first().unwrap() - next;
    });
    line.first().unwrap() - next
}
