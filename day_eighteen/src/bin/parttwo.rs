#[derive(Debug)]
struct Entry {
    direction: (i128, i128),
    distance: i128,
}

fn main() {
    let input: Vec<Entry> = include_str!("input").lines().map(|x| {
        let color = x.split_whitespace().last().unwrap();
        let color = color.strip_prefix('(').unwrap().strip_suffix(')').unwrap().chars().collect::<Vec<char>>();
        let direction = match &color.last().unwrap() {
            '0' => (1, 0),
            '1' => (0, 1),
            '2' => (-1, 0),
            '3' => (0, -1),
            _ => panic!("Invalid direction"),
        };
        Entry {
            direction,
            distance: hexadecimal_decimal(&color[1..color.len()-1].iter().collect::<String>()),
        }
    }).collect::<Vec<Entry>>();

    println!("{:?}", input);

    let mut polygons: Vec<(i128, i128)> = vec![(0, 0)];
    let mut point_count = 0i128;

    // use the shoelace algorithm
    for line in input{
        let last_point = *polygons.last().unwrap();
        let direction = line.direction;
        let distance = line.distance;
        point_count += distance;
        polygons.push((last_point.0 + direction.0 * distance, last_point.1 + direction.1 * distance));
    }

    println!("{:?}", polygons);

    let mut area = 0;
    for i in 0..polygons.len(){
        let curr_point = polygons[i];
        let next_point = polygons[(i + 1) % polygons.len()];
        area += (curr_point.0 * next_point.1) - (curr_point.1 * next_point.0);
    }

    let total = (area.abs() / 2) + point_count / 2 + 1;
    println!("{}", total);
}

// did it manually just for the fun of it
fn hexadecimal_decimal(num: &str)->i128{
    i128::from_str_radix(num, 16).unwrap()
}