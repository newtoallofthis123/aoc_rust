struct Entry {
    direction: (i128, i128),
    distance: i128,
}

fn main() {
    let input = include_str!("input").lines().map(|x| {
        let split = x.split_whitespace().collect::<Vec<&str>>();
        let direction = match split[0] {
            "U" => (0, -1),
            "D" => (0, 1),
            "L" => (-1, 0),
            "R" => (1, 0),
            _ => panic!("Invalid direction"),
        };
        Entry {
            direction,
            distance: split[1].parse::<i128>().unwrap(),
        }
    }).collect::<Vec<Entry>>();

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
