#[derive(Debug, Clone)]
struct Card {
    number: u32,
    winning: Vec<u32>,
    having: Vec<u32>,
}

fn main() {
    let input = include_str!("temp").lines().collect::<Vec<&str>>();
    let cards = input.iter().map(|line| tokenize(line)).collect::<Vec<Card>>();

    let mut total = 0;

    for card in cards{
        let winning = get_winning(card.clone());
        let points = get_points(winning);
        total += points;
        println!("{:?}", &card);
        println!("Card {}: {:?}", card.number, points);
    }

    println!("Total: {}", total);
}

fn tokenize(line: &str) -> Card {
    let line = line.to_string();
    let mut winning = Vec::new();
    let mut having = Vec::new();

    let card_number = &line.split(':').collect::<Vec<&str>>()[0]
        .strip_prefix("Card ")
        .expect("No card number")
        .replace(' ', "")
        .parse::<u32>()
        .expect("Not a number");

    let nums = line
        .split(':')
        .collect::<Vec<&str>>()[1]
        .split(' ')
        .collect::<Vec<&str>>();

    let mut pos = 0;

    for (i, num) in nums.clone().iter().enumerate() {
        if num == &"|"{
            pos = i;
            break;
        }
        if num.parse::<u32>().is_ok() {
            winning.push(num.parse::<u32>().unwrap());
        }
    }

    for num in nums[pos+1..].iter(){
        if num.parse::<u32>().is_ok() {
            having.push(num.parse::<u32>().unwrap());
        }
    }

    Card{
        number: *card_number,
        winning,
        having,
    }
}

fn get_winning(card: Card)->Vec<u32>{
    let mut winning = Vec::new();

    for num in card.winning{
        if card.having.contains(&num){
            winning.push(num);
        }
    }

    winning
}

fn get_points(points: Vec<u32>)->u32{
    let length = points.len();

    if points.is_empty(){
        return 0;
    }

    2u32.pow(length as u32 - 1)
}