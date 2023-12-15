fn main() {
    let sequence = include_str!("input").split(',').collect::<Vec<_>>();

    let total = sequence.iter().map(|x| calc_hash(x)).sum::<u128>();

    println!("Total: {}", total);
}

fn calc_hash(str: &str) -> u128 {
    let mut curr_val: u128 = 0;

    for ch in str.chars() {
        curr_val += ch as u128;
        curr_val *= 17;
        curr_val %= 256;
    }

    curr_val
}

#[test]
fn is_correct() {
    let res = 1320;
    let string = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
    let sequence = string.split(',').collect::<Vec<_>>();
    let total = sequence.iter().map(|x| calc_hash(x)).sum::<u128>();

    assert_eq!(res, total);
}
