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

#[derive(Clone)]
struct Range {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Range {
    fn get_combinations(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }
}

fn valid_ranges(curr_range: Range, workflow: String, workflows: &HashMap<String, Workflow>) -> Vec<Range> {
    if workflow == "A"{
        return vec![curr_range];
    } 
    if workflow == "R"{
        return vec![];
    }

    let mut valid = Vec::new();
    let mut curr_range = curr_range.clone();
    let curr_workflow = workflows.get(&workflow).unwrap();

    for rule in &curr_workflow.rules{
        let des = rule.next.clone();
        match rule.gear{
            Type::X => match rule.compare{
                Compare::LessThan =>{
                    if curr_range.x.1 < rule.num{
                        valid.extend(valid_ranges(curr_range.clone(), des, workflows));
                    } else if curr_range.x.0 < rule.num{
                        valid.extend(valid_ranges(
                            Range{
                                x: (curr_range.x.0, rule.num - 1),
                                m: curr_range.m,
                                a: curr_range.a,
                                s: curr_range.s,
                            },
                            des,
                            workflows,
                        ));

                        curr_range.x.0 = rule.num;
                    }
                }
                Compare::GreaterThan => {
                    if curr_range.x.0 > rule.num {
                        valid.extend(valid_ranges(curr_range.clone(), des, workflows))
                    } else if curr_range.x.1 > rule.num {
                        valid.extend(valid_ranges(
                            Range {
                                x: (rule.num + 1, curr_range.x.1),
                                m: curr_range.m,
                                a: curr_range.a,
                                s: curr_range.s,
                            },
                            des,
                            workflows,
                        ));
                        curr_range.x.1 = rule.num;
                    }
                }
                Compare::Default => {
                    valid.extend(valid_ranges(curr_range.clone(), des, workflows));
                }
            }
            Type::M => {
                match rule.compare{
                    Compare::LessThan =>{
                        if curr_range.m.1 < rule.num{
                            valid.extend(valid_ranges(curr_range.clone(), des, workflows));
                        } else if curr_range.m.0 < rule.num{
                            valid.extend(valid_ranges(
                                Range{
                                    x: curr_range.x,
                                    m: (curr_range.m.0, rule.num - 1),
                                    a: curr_range.a,
                                    s: curr_range.s,
                                },
                                des,
                                workflows,
                            ));
    
                            curr_range.m.0 = rule.num;
                        }
                    }
                    Compare::GreaterThan => {
                        if curr_range.m.0 > rule.num {
                            valid.extend(valid_ranges(curr_range.clone(), des, workflows))
                        } else if curr_range.m.1 > rule.num {
                            valid.extend(valid_ranges(
                                Range {
                                    x: curr_range.x,
                                    m: (rule.num + 1, curr_range.m.1),
                                    a: curr_range.a,
                                    s: curr_range.s,
                                },
                                des,
                                workflows,
                            ));
                            curr_range.m.1 = rule.num;
                        }
                    }
                    Compare::Default => {
                        valid.extend(valid_ranges(curr_range.clone(), des, workflows));
                    }
                }
            }
            Type::A => {
                match rule.compare{
                    Compare::LessThan =>{
                        if curr_range.a.1 < rule.num{
                            valid.extend(valid_ranges(curr_range.clone(), des, workflows));
                        } else if curr_range.a.0 < rule.num{
                            valid.extend(valid_ranges(
                                Range{
                                    x: curr_range.x,
                                    m: curr_range.m,
                                    a: (curr_range.a.0, rule.num - 1),
                                    s: curr_range.s,
                                },
                                des,
                                workflows,
                            ));
    
                            curr_range.a.0 = rule.num;
                        }
                    }
                    Compare::GreaterThan => {
                        if curr_range.a.0 > rule.num {
                            valid.extend(valid_ranges(curr_range.clone(), des, workflows))
                        } else if curr_range.a.1 > rule.num {
                            valid.extend(valid_ranges(
                                Range {
                                    x: curr_range.x,
                                    m: curr_range.m,
                                    a: (rule.num + 1, curr_range.a.1),
                                    s: curr_range.s,
                                },
                                des,
                                workflows,
                            ));
                            curr_range.a.1 = rule.num;
                        }
                    }
                    Compare::Default => {
                        valid.extend(valid_ranges(curr_range.clone(), des, workflows));
                    }
                }
            }
            Type::S => {
                match rule.compare{
                    Compare::LessThan =>{
                        if curr_range.s.1 < rule.num{
                            valid.extend(valid_ranges(curr_range.clone(), des, workflows));
                        } else if curr_range.s.0 < rule.num{
                            valid.extend(valid_ranges(
                                Range{
                                    x: curr_range.x,
                                    m: curr_range.m,
                                    a: curr_range.a,
                                    s: (curr_range.s.0, rule.num - 1),
                                },
                                des,
                                workflows,
                            ));
    
                            curr_range.s.0 = rule.num;
                        }
                    }
                    Compare::GreaterThan => {
                        if curr_range.s.0 > rule.num {
                            valid.extend(valid_ranges(curr_range.clone(), des, workflows))
                        } else if curr_range.s.1 > rule.num {
                            valid.extend(valid_ranges(
                                Range {
                                    x: curr_range.x,
                                    m: curr_range.m,
                                    a: curr_range.a,
                                    s: (rule.num + 1, curr_range.s.1),
                                },
                                des,
                                workflows,
                            ));
                            curr_range.s.1 = rule.num;
                        }
                    }
                    Compare::Default => {
                        valid.extend(valid_ranges(curr_range.clone(), des, workflows));
                    }
                }
            }
        }
    }

    valid
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
                .map(|rule| match rule.find(':') {
                    Some(_) => {
                        let rule: Vec<_> = rule.split(':').collect();
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
                        next: rule.to_string(),
                    },
                })
                .collect();
            (name.to_string(), Workflow { rules: conditions })
        })
        .collect::<HashMap<_, _>>();

    let total =  valid_ranges(
        Range {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        "in".to_string(),
        &workflows,
    )
    .into_iter()
    .map(|range| {
        let range = range.get_combinations();
        println!("{}", range);
        range
    })
    .sum::<usize>();

    println!("{}", total);
}
