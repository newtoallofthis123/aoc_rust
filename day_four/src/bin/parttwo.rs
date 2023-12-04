fn main() {
    let input = include_str!("temp").lines().collect::<Vec<&str>>();

    let mut card_count: Vec<u32> = vec![1; input.len()];
    println!("{:?}", card_count);

    for (card, line) in input.iter().enumerate() {
        let nums_part = line
            .split(':')
            .last()
            .unwrap()
            .split('|')
            .collect::<Vec<&str>>()
            .iter()
            .map(|x| x.split(' ').collect::<Vec<&str>>())
            .collect::<Vec<Vec<&str>>>();
        
        let mut winning = Vec::new();
        let mut having = Vec::new();

        for num in nums_part[0].iter() {
            if num.parse::<u32>().is_ok() {
                winning.push(num.parse::<u32>().unwrap());
            }
        }

        for num in nums_part[1].iter() {
            if num.parse::<u32>().is_ok() {
                having.push(num.parse::<u32>().unwrap());
            }
        }

        let count = winning.into_iter().filter(|x| having.contains(x)).count();

        println!("Card {}: {}", card, count);
        if count != 0{
            for i in card + 1..=card + count {
                card_count[i] += card_count[card];
                println!("{:?}", card_count);
            }
        }
    }

    let total = card_count.into_iter().sum::<u32>();
    println!("Total: {}", total);
}
