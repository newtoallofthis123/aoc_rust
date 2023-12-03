// Wrong one for part two
fn main(){
    let input = get_input();

    let mut nums = Vec::new();

    for i in 0..input.len()-2{

        println!("{} {}", i, input.len()-2);

        if i == input.len() - 3{
            break;
        }

        let valid = valid_nums(&input[i..i+3]);
        if valid.is_none(){
            continue;
        }
        let valid_nums = valid.unwrap();
        for num in valid_nums{
            nums.push(num);
        }
    }

    let lines = vec![boiler(), input[0].clone(), input[1].clone()];
    let valid = valid_nums(&[lines[0].clone(), lines[1].clone(), lines[2].clone()]);

    if valid.is_some(){
        let valid_nums = valid.unwrap();
        for num in valid_nums{
            nums.push(num);
        }
    }

    let lines = vec![input[input.len()-2].clone(), input[input.len()-1].clone(), boiler()];
    let valid = valid_nums(&[lines[0].clone(), lines[1].clone(), lines[2].clone()]);
    if valid.is_some(){
        let valid_nums = valid.unwrap();
        for num in valid_nums{
            nums.push(num);
        }
    }

    let mut total = 0;
    for num in nums{
        total += num;
    }

    println!("{}", total);
}

fn boiler()->String{
    "..........".to_string()
}

fn get_input()->Vec<String>{
    let input = std::fs::read_to_string("input").unwrap();

    input.lines().map(|x| x.to_string()).collect()
}

fn tokenize(line: &str)->Vec<char>{
    line.chars().collect()
}

fn does_contain_numbers(line: &str)->bool{
    line.chars().any(|x| x.is_numeric())
}

fn is_special_char(c: char)->bool{
    !c.is_numeric() && c != '.'
}

fn get_num_positions(line: &str)->(Vec<Vec<char>>, Vec<Vec<usize>>){
    let mut nums = Vec::new();
    let mut positions = Vec::new();

    let mut temp = Vec::new();
    let mut temp_positions = Vec::new();

    for (i, c) in line.chars().enumerate(){
        if c.is_numeric(){
            temp.push(c);
            temp_positions.push(i);
        }
        else if !c.is_numeric(){
            if temp.is_empty() || temp_positions.is_empty(){
                continue;
            }
            nums.push(temp.clone());
            positions.push(temp_positions.clone());
            temp.clear();
            temp_positions.clear();
        }
        if i == line.len() - 1 && !temp.is_empty() && !temp_positions.is_empty() {
            nums.push(temp.clone());
            positions.push(temp_positions.clone());
            temp.clear();
            temp_positions.clear();
        } 
    }

    (nums, positions)
}

fn get_imp_positions(points: Vec<Vec<usize>>, length: usize)->Vec<Vec<usize>>{
    let mut imp_positions = Vec::new();

    for point in points{
        let mut temp = Vec::new();
        let first = point[0];
        let last = point[point.len() - 1];
        if first == 0{
            temp.push(first);
        }
        else{
            temp.push(first - 1);
            temp.push(first);
        }
        (1..point.len() - 1).for_each(|i| {
            temp.push(point[i]);
        });
        if last == length-1{
            temp.push(last);
        }
        else{
            temp.push(last);
            temp.push(last + 1);
        }
        imp_positions.push(temp);
    }

    imp_positions
}

fn valid_nums(lines: &[String])->Option<Vec<i32>>{

    let mut result = Vec::new();

    let (prev, curr, next) = (tokenize(&lines[0]), tokenize(&lines[1]), tokenize(&lines[2]));


    if !does_contain_numbers(curr.iter().collect::<String>().as_str()){
        return None;
    }

    println!("{:?}", prev);
    println!("{:?}", curr);
    println!("{:?}", next);

    println!("{:?}", get_num_positions(&lines[1]));

    let (nums, pos) = get_num_positions(&lines[1]);

    println!("{:?}", get_imp_positions(pos.clone(), curr.len()));

    for (i, position) in get_imp_positions(pos, curr.len()).iter().enumerate(){
        if has_special_char(&lines[0], position) || has_special_char(&lines[2], position) || check_sides(&lines[1], position){
            //join the number of i index in nums
            let res = nums[i].iter().collect::<String>().parse::<i32>().unwrap();
            result.push(res);
        }
    }

    Some(result)
}

fn check_sides(line:&str, positions : &[usize])->bool{
    let first = line.chars().nth(positions[0]).unwrap_or('.');
    let last = line.chars().nth(positions[positions.len() - 1]).unwrap_or('.');
    
    is_special_char(first) || is_special_char(last)
}

fn has_special_char(line: &str, positions : &[usize])->bool{
    positions.iter().any(|x| is_special_char(line.chars().nth(*x).unwrap_or('.')))
}