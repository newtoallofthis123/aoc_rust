use std::collections::HashMap;

fn main(){
    let input = get_input();

    let mut total = 0;

    for line in input {
        let (game_id, min) = get_result(line);
        println!("{} {:?} {}", game_id, min, get_mul(&min));
        total += get_mul(&min);
    }

    println!("Total: {}", total);
}

fn get_mul(min: &HashMap<Ball, i32>) -> i32 {
    let mut mul = 1;

    for (_, count) in min.iter() {
        mul *= count;
    }

    mul
}

fn get_input() -> Vec<String> {
    std::fs::read_to_string("input")
        .expect("Unable to read file")
        .lines()
        .map(|line| line.to_string())
        .collect()
}

fn get_result(line: String) -> (i32, HashMap<Ball, i32>) {

    let separated = line.split(':').collect::<Vec<&str>>();

    let game_id = separated[0].split(' ').collect::<Vec<&str>>()[1]
        .parse::<i32>()
        .unwrap();

    let games = separated[1].split(';').collect::<Vec<&str>>();

    (game_id, get_minimum(games))
}

fn get_minimum(games: Vec<&str>) -> HashMap<Ball, i32> {
    let mut min = HashMap::new();

    let mut red_max = 0;
    let mut blue_max = 0;
    let mut green_max = 0;

    for game in games {
        let balls = get_game(game);

        let red = balls.get(&Ball::Red).unwrap_or(&0);
        let blue = balls.get(&Ball::Blue).unwrap_or(&0);
        let green = balls.get(&Ball::Green).unwrap_or(&0);

        if red > &red_max {
            red_max = *red;
        }

        if blue > &blue_max {
            blue_max = *blue;
        }

        if green > &green_max {
            green_max = *green;
        }
    }

    min.insert(Ball::Red, red_max);
    min.insert(Ball::Blue, blue_max);
    min.insert(Ball::Green, green_max);

    min
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
