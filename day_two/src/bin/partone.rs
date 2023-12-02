use std::collections::HashMap;

fn main() {
    let input = get_input();

    let mut total = 0;

    for line in input {
        let (game_id, possible) = get_result(line);
        println!("{} {}", game_id, possible);
        if possible {
            total += game_id;
        }
    }

    println!("Total: {}", total);
}

fn get_input() -> Vec<String> {
    std::fs::read_to_string("input")
        .expect("Unable to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn get_result(line: String) -> (i32, bool) {

    let separated = line.split(':').collect::<Vec<&str>>();

    let game_id = separated[0].split(' ').collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let games = separated[1].split(';').collect::<Vec<&str>>();

    for game in games {
        if !is_possible(game) {
            return (game_id, false);
        }
    }

    (game_id, true)
}

fn is_possible(game: &str)->bool{
    let balls = get_game(game);

    let result = get_game(" 12 red, 13 green, 14 blue");

    for (ball, count) in balls.iter() {
        if count > result.get(ball).unwrap() {
            return false;
        }
    }
    
    true
}

fn get_game(game: &str) -> HashMap<Ball, i32> {
    let game_temp = game
        .split(' ')
        .map(|s| s.replace([',', '(', ')'], ""))
        .collect::<Vec<String>>();

    let mut game_map = HashMap::new();

    (1..game_temp.len()).for_each(|i| {
        if i % 2 == 0 {
            let ball = match game_temp[i].as_str() {
                "red" => Ball::Red,
                "blue" => Ball::Blue,
                "green" => Ball::Green,
                _ => {
                    println!("{}", game_temp[i]);
                    panic!("Invalid ball color")
                },
            };
            game_map.insert(ball, game_temp[i - 1].parse::<i32>().unwrap());
        }
    });

    game_map
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Ball {
    Red,
    Blue,
    Green,
}
