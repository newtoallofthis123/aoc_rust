#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Card {
    Ace,
    Number(u8),
    Jack,
    Queen,
    King,
    T,
}

impl Card {
    fn new(input: String) -> Vec<Card> {
        let mut cards = Vec::new();
        for c in input.chars() {
            if c.is_numeric() {
                cards.push(Card::Number(c.to_digit(10).unwrap() as u8));
            } else {
                match c {
                    'K' => cards.push(Card::King),
                    'Q' => cards.push(Card::Queen),
                    'J' => cards.push(Card::Jack),
                    'T' => cards.push(Card::T),
                    'A' => cards.push(Card::Ace),
                    _ => panic!("Some Weird Character {} found!", c),
                }
            }
        }

        cards
    }
}

// Implement this for PartialOrd
impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (&Self::Ace, &Self::Ace) => Some(std::cmp::Ordering::Equal),
            (&Self::King, &Self::King) => Some(std::cmp::Ordering::Equal),
            (&Self::Queen, &Self::Queen) => Some(std::cmp::Ordering::Equal),
            (&Self::Jack, &Self::Jack) => Some(std::cmp::Ordering::Equal),
            (&Self::T, &Self::T) => Some(std::cmp::Ordering::Equal),
            (Self::Ace, _) => Some(std::cmp::Ordering::Greater),
            (_, Self::Ace) => Some(std::cmp::Ordering::Less),
            (Self::King, _) => Some(std::cmp::Ordering::Greater),
            (_, Self::King) => Some(std::cmp::Ordering::Less),
            (Self::Queen, _) => Some(std::cmp::Ordering::Greater),
            (_, Self::Queen) => Some(std::cmp::Ordering::Less),
            (Self::Jack, _) => Some(std::cmp::Ordering::Greater),
            (_, Self::Jack) => Some(std::cmp::Ordering::Less),
            (Self::T, _) => Some(std::cmp::Ordering::Greater),
            (_, Self::T) => Some(std::cmp::Ordering::Less),
            (Self::Number(x), Self::Number(y)) => x.partial_cmp(y),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Hand {
    FiveKind(Vec<Card>),
    FourKind(Vec<Card>),
    FullHouse(Vec<Card>),
    ThreeKind(Vec<Card>),
    TwoPair(Vec<Card>),
    OnePair(Vec<Card>),
    HighCard(Vec<Card>),
}

impl Hand {
    fn new(card: Vec<Card>) -> Self {
        let (dup_cards, distinct) = Self::get_same_ones(card.clone());
        let cards_len = dup_cards.len();

        match (cards_len, distinct) {
            (5, 2) => Self::FullHouse(card),
            (5, _) => Self::FiveKind(card),
            (3, _) => Self::ThreeKind(card),
            (4, 2) => Self::TwoPair(card),
            (4, _) => Self::FourKind(card),
            (2, _) => Self::OnePair(card),
            (_, _) => Self::HighCard(card),
        }
    }

    fn get_same_ones(card: Vec<Card>) -> (Vec<Card>, i32) {
        let mut same_ones = Vec::new();
        let mut distinct = 0;
        for c in card.iter() {
            if card.iter().filter(|x| *x == c).count() > 1 {
                if !same_ones.contains(c) {
                    distinct += 1;
                }
                same_ones.push(c.to_owned());
            }
        }

        (same_ones, distinct)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::FiveKind(x), Self::FiveKind(y)) => greater_one(x.to_vec(), y.to_vec()),
            (Self::FourKind(x), Self::FourKind(y)) => greater_one(x.to_vec(), y.to_vec()),
            (Self::FullHouse(x), Self::FullHouse(y)) => greater_one(x.to_vec(), y.to_vec()),
            (Self::ThreeKind(x), Self::ThreeKind(y)) => greater_one(x.to_vec(), y.to_vec()),
            (Self::TwoPair(x), Self::TwoPair(y)) => greater_one(x.to_vec(), y.to_vec()),
            (Self::OnePair(x), Self::OnePair(y)) => greater_one(x.to_vec(), y.to_vec()),
            (Self::HighCard(x), Self::HighCard(y)) => greater_one(x.to_vec(), y.to_vec()),
            (Self::FiveKind(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Self::FiveKind(_)) => Some(std::cmp::Ordering::Less),
            (Self::FourKind(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Self::FourKind(_)) => Some(std::cmp::Ordering::Less),
            (Self::FullHouse(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Self::FullHouse(_)) => Some(std::cmp::Ordering::Less),
            (Self::ThreeKind(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Self::ThreeKind(_)) => Some(std::cmp::Ordering::Less),
            (Self::TwoPair(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Self::TwoPair(_)) => Some(std::cmp::Ordering::Less),
            (Self::OnePair(_), _) => Some(std::cmp::Ordering::Greater),
            (_, Self::OnePair(_)) => Some(std::cmp::Ordering::Less),
        }
    }
}

fn greater_one(first: Vec<Card>, second: Vec<Card>) -> Option<std::cmp::Ordering> {
    for (f, s) in first.iter().zip(second.iter()) {
        if f > s {
            return Some(std::cmp::Ordering::Greater);
        } else if f < s {
            return Some(std::cmp::Ordering::Less);
        } else {
            continue;
        }
    }

    None
}

fn main() {
    let inputs = include_str!("input")
        .lines()
        .map(|x| {
            let line = x.split(' ').collect::<Vec<&str>>();
            let hand = Hand::new(Card::new(line[0].to_string()));
            let bid = line[1].parse::<i32>().unwrap();
            (hand, bid)
        })
        .collect::<Vec<(Hand, i32)>>();

    println!("{:?}", inputs);

    let ordered = order_hands(&mut inputs.to_vec());

    let mut total = 0;

    for (i, pair) in ordered.iter().rev().enumerate() {
        println!("{:?} * {}", pair, i + 1);
        total += pair.1 * (i as i32 + 1);
    }

    println!("Total: {}", total);
}

fn order_hands(hands: &mut Vec<(Hand, i32)>) -> Vec<(Hand, i32)> {
    for i in 0..hands.len() {
        for j in 0..hands.len() {
            if hands[i].0 > hands[j].0 {
                // println!("{:?} > {:?}", hands[i], hands[j]);
                hands.swap(i, j);
            }
        }
    }

    hands.to_vec()
}
