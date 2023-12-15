/// This was the old approach before updating it for the new requirements.
/// This is my original approach, so feel free to roast me for it.

fn main(){

    let input = include_str!("input").lines().map(|x| x.to_string()).collect::<Vec<String>>();

    let transposed = transpose(input);
    let mut res = Vec::new();
    for line in transposed{
        let mut temp = line.chars().rev().collect::<Vec<char>>();
        let len = temp.len();
        for j in 0..len{
            for k in j..len{
                if k == len-1{
                    break;
                }
                if temp[k] == 'O' && temp[k+1] != '#'{
                    temp.swap(k, k+1)
                }
            }
        }
        res.push(temp.iter().rev().collect::<String>());
    }

    let mut total = 0;
    let len = transpose(res.clone()).len();

    for (i, line) in transpose(res.clone()).iter().enumerate(){
        let to_add = line.chars().filter(|&x| x == 'O').count() * (len - i);
        println!("{} {} {}", line, len-i, to_add);
        total += to_add;
    }

    println!("{}", total);
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
