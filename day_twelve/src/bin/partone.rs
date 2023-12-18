fn main(){
    let input = include_str!("temp").lines().map(|x| {
        let t = x.split_whitespace().collect::<Vec<_>>();
        let nums = t[1].split(',').map(|n| n.parse::<u8>().unwrap()).collect::<Vec<_>>();

        (t[0], nums)
    }).collect::<Vec<_>>();

    let test = &input[0];
    let (pattern, nums) = test.clone();

    find_arrangements(pattern, nums);
}

fn find_arrangements(pattern: &str, nums: Vec<u8>)->u16{
    let mut queue = nums;
    let mut arrangements = 0;
    queue.sort();
    let mut pattern: Vec<char> = pattern.chars().collect::<Vec<char>>();

    while let Some(num) = queue.pop(){
        queue.sort();

        let mut slice = Vec::new();

        

        println!("P: {:?}", pattern);
        println!("S: {:?}", slice);
    }
    0
}