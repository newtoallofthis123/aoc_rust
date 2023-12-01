fn main() {
    let inputs = get_inputs();

    let mut total = 0;

    for word in inputs{
        total += process_word(word.clone());
        println!("{}: {}", word, process_word(word.clone()));
    }

    println!("Total: {}", total);
}

fn get_inputs()->Vec<String>{
    let input = std::fs::read_to_string("input").expect("input not found");
    input.split('\n').map(|x| x.to_string()).collect()
}

fn process_word(word: String)->i32{
    let mut result: Vec<char> = Vec::new();
    let mut answer: Vec<char> = Vec::new();
    for c in word.chars(){
        if c.is_numeric(){
            result.push(c);
        }
    }

    if result.len() == 1{
        result.push(result.clone().pop().unwrap());
    }

    // put first char and last char in answer
    answer.push(result[0]);
    answer.push(result[result.len()-1]);

    answer.iter().collect::<String>().parse::<i32>().unwrap()
}

// Total: 53334