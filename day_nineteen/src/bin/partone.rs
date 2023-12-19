// Did it on my own first, but it was too long and messy
// So I decided to use some ideas from this snippet (I didn't copy it, I just used some of the ideas)
// https://github.com/tymscar/Advent-Of-Code/blob/master/2023/rust/src/day19/part1.rs
// Thanks @tymscar

use std::collections::HashMap;

#[derive(Clone, Copy)]
enum Compare {
    LessThan,
    GreaterThan,
    Default,
}

#[derive(Clone, Copy)]
enum Type {
    X,
    M,
    A,
    S,
}

#[derive(Clone)]
struct Condition {
    compare: Compare,
    gear: Type,
    num: usize,
    next: String,
}

struct Workflow {
    rules: Vec<Condition>,
}

struct Rating {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Rating {
    fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool {
        let mut curr_workflow = workflows.get("in").unwrap();
        loop {
            let mut destination: String = "R".to_string();
            for rule in &curr_workflow.rules {
                match rule.gear {
                    Type::X => match rule.compare {
                        Compare::LessThan => {
                            if self.x < rule.num {
                                destination = rule.next.clone();
                                break;
                            }
                        }
                        Compare::GreaterThan => {
                            if self.x > rule.num {
                                destination = rule.next.clone();
                                break;
                            }
                        }
                        Compare::Default => {
                            destination = rule.next.clone();
                        }
                    },

                    Type::M => match rule.compare {
                        Compare::LessThan => {
                            if self.m < rule.num {
                                destination = rule.next.clone();
                                break;
                            }
                        }
                        Compare::GreaterThan => {
                            if self.m > rule.num {
                                destination = rule.next.clone();
                                break;
                            }
                        }
                        Compare::Default => {
                            destination = rule.next.clone();
                        }
                    },
                    Type::A => match rule.compare {
                        Compare::LessThan => {
                            if self.a < rule.num {
                                destination = rule.next.clone();
                                break;
                            }
                        }
                        Compare::GreaterThan => {
                            if self.a > rule.num {
                                destination = rule.next.clone();
                                break;
                            }
                        }
                        Compare::Default => {
                            destination = rule.next.clone();
                        }
                    },
                    Type::S => match rule.compare {
                        Compare::LessThan => {
                            if self.s < rule.num {
                                destination = rule.next.clone();
                                break;
                            }
                        }
                        Compare::GreaterThan => {
                            if self.s > rule.num {
                                destination = rule.next.clone();
                                break;
                            }
                        }
                        Compare::Default => {
                            destination = rule.next.clone();
                        }
                    },
                }
            }
            match destination.chars().next().unwrap() {
                'A' => return true,
                'R' => return false,
                _ => {
                    curr_workflow = workflows.get(&destination).unwrap();
                }
            }
        }
    }

    fn sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

fn main() {
    let lines: Vec<_> = include_str!("input").split("\n\n").collect();
    let workflows = lines[0]
        .lines()
        .map(|line| {
            let line = line.split('{').collect::<Vec<_>>();
            let name = line[0].trim().to_string();
            let conditions = line[1][..line[1].len() - 1]
                .split(',')
                .map(|condition| match condition.find(':') {
                    Some(_) => {
                        let rule: Vec<_> = condition.split(':').collect();
                        let destination: String = rule[1].trim().to_string();
                        let kind: Compare = match rule[0].chars().nth(1).unwrap() {
                            '<' => Compare::LessThan,
                            '>' => Compare::GreaterThan,
                            _ => panic!("Invalid rule {}", rule[0]),
                        };
                        let subject: Type = match rule[0].chars().next().unwrap() {
                            'x' => Type::X,
                            'm' => Type::M,
                            'a' => Type::A,
                            's' => Type::S,
                            _ => panic!("Invalid rule {}", rule[0]),
                        };
                        let num: usize = rule[0][2..].parse().unwrap();
                        Condition {
                            compare: kind,
                            gear: subject,
                            num,
                            next: destination,
                        }
                    }
                    None => Condition {
                        compare: Compare::Default,
                        gear: Type::X,
                        num: 0,
                        next: condition.to_string(),
                    },
                })
                .collect();
            (name.to_string(), Workflow { rules: conditions })
        })
        .collect();

    let ratings: Vec<Rating> = lines[1]
        .lines()
        .map(|line| {
            let line = line[1..line.len() - 1].split(',').collect::<Vec<_>>();
            let x = line[0][2..].parse().unwrap();
            let m = line[1][2..].parse().unwrap();
            let a = line[2][2..].parse().unwrap();
            let s = line[3][2..].parse().unwrap();
            Rating { x, m, a, s }
        })
        .collect();

    let total = ratings
        .iter()
        .filter_map(|rating| {
            if rating.is_accepted(&workflows) {
                Some(rating.sum())
            } else {
                None
            }
        })
        .sum::<usize>();

    println!("{}", total);
}
