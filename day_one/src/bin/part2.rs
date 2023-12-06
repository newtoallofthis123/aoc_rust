fn main(){
    let inputs = get_inputs();

    let mut total = 0;

    for word in inputs{
        total += get_word(word.clone());
        println!("{}: {}", word, get_word(word.clone()));
    }

    println!("Total: {}", total);
}

fn get_word(word: String)->i32{
    let words = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let mut results: Vec<char> = Vec::new();
    let mut answer: Vec<char> = Vec::new();

    let mut i =0;
    let mut temp_word = String::new();
    while i <= word.len() {
        let c = word.chars().nth(i).unwrap();
        // println!("{}: {}", i, c);
        if c.is_alphabetic(){
            temp_word.push(c);
        }
        else if c.is_numeric(){
            results.push(c);
        }

        println!("temp_word: {}", temp_word);
        
        for w in words.iter(){
            if temp_word.contains(w){
                results.push(convert_to_number(w));
                temp_word.clear();
            }
        }

        if i != word.len()-1{
            i += 1;
        }
        else{
            break;
        }
    }

    if results.len() == 1{
        results.push(results.clone().pop().unwrap());
    }

    // put first char and last char in answer
    answer.push(results[0]);
    answer.push(results[results.len()-1]);

    answer.iter().collect::<String>().parse::<i32>().unwrap()

}

fn convert_to_number(word: &str)->char{
    match word {
        "zero" => '0',
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => ' ', 
    }
}

fn get_inputs()->Vec<String>{
    let input = std::fs::read_to_string("input").expect("input not found");
    input.split('\n').map(|x| x.to_string()).collect()
}

// Total: 52834