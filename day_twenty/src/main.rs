use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
enum Module {
    FlipFlop(String, bool, Vec<String>),
    Conjunction(String, Vec<bool>, Vec<String>),
}

impl Module {
    fn process(&mut self, input: bool) -> Option<(Vec<String>, bool)> {
        match self {
            Self::FlipFlop(_, state, des) => match input {
                false => Some((
                    des.clone(),
                    match state {
                        true => {
                            *state = false;
                            false
                        }
                        false => {
                            *state = true;
                            true
                        }
                    },
                )),
                true => None,
            },
            Self::Conjunction(_, states, des) => {
                if states.is_empty() {
                    states.push(false); // Default to remembering a low pulse
                }

                // Assuming 'input' is a boolean representing the type of pulse received
                states.pop(); // Remove the previous state
                states.push(input); // Update the memory for the current input

                println!("states: {:#?}", states);

                // Check if it remembers high pulses for all inputs
                let valid = states.iter().all(|&x| x);

                // Send a low pulse if it remembers high pulses for all inputs, otherwise send a high pulse
                Some((des.to_owned(), !valid))
            }
        }
    }
}

fn main() {
    let mut queue = VecDeque::new();
    let mut modules = HashMap::new();
    let button_presses: u16 = 1000;

    include_str!("input").lines().for_each(|x| {
        let split = x.split(" -> ").collect::<Vec<_>>();
        let name = split[0].to_string();
        let to = split[1]
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<_>>();
        if name.starts_with('%') {
            modules.insert(
                name.strip_prefix('%').unwrap().to_string(),
                Module::FlipFlop(name.strip_prefix('%').unwrap().to_string(), false, to),
            );
        } else if name.starts_with('&') {
            modules.insert(
                name.strip_prefix('&').unwrap().to_string(),
                Module::Conjunction(
                    name.strip_prefix('&').unwrap().to_string(),
                    vec![false; to.len()],
                    to,
                ),
            );
        } else if name.trim() == "broadcaster" {
            to.iter().for_each(|x| queue.push_back((x.clone(), false)))
        } else {
            println!("wut {}", name);
            panic!("wut this module?")
        }
    });

    println!("{:#?}", queue);

    let mut low: Vec<bool> = Vec::new();
    // queue.clone().iter().for_each(|_| low.push(false));
    let og_queue = queue.clone();
    let mut high: Vec<bool> = Vec::new();

    // for _ in 0..button_presses {
    //     let (name, input) = ("c".to_string(), true);
    //     let (des, output) = modules.get_mut(&name).unwrap().process(input);
    //     println!("Q before: {:#?}", queue);
    //     println!("Des: {:#?}", des);
    //     if let Some(des) = des {
    //         des.iter().for_each(|x| queue.push((x.clone(), output)));
    //     }
    //     println!("Q After: {:#?}", queue);
    //     println!("{}: {} -> {}", name, input, output);
    // }

    for _ in 0..button_presses {
        queue = og_queue.clone();
        low.push(false);
        queue.clone().iter().for_each(|_| low.push(false));
        while let Some(module) = queue.pop_front() {
            println!("Q: {:#?}", queue);
            let (name, input) = module;
            if let Some(des) = modules.get_mut(&name).expect("Not found dude").process(input) {
                let output = des.1;
                des.0
                    .iter()
                    .for_each(|x| {
                        if modules.get(x).is_some() {
                            queue.push_back((x.clone(), output))
                        }
                    });
                println!("Des: {:#?}", des);
                if output {
                    high.push(output);
                } else {
                    low.push(output);
                }
                println!("{}: {} -> {}", name, input, output);
            }
        }
    }

    println!("low: {}, high: {}", low.len(), high.len());
    println!("total: {:#?}", low.len() * high.len());
}
