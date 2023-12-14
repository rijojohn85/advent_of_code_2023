use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
#[derive(Debug, PartialEq, Eq)]
#[allow(dead_code)]
enum Hand {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

// const HAND_SCORE: [Hand; 7] = [
//     Hand::HighCard,
//     Hand::OnePair,
//     Hand::TwoPair,
//     Hand::ThreeOfAKind,
//     Hand::FullHouse,
//     Hand::FourOfAKind,
//     Hand::FiveOfAKind,
// ];
const CARD_SCORE: [char; 13] = [
    '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

fn main() {
    let reader = BufReader::new(File::open("input_day7").unwrap());
    let mut high_card: Vec<(String, Hand, usize)> = vec![];
    let mut one_pair: Vec<(String, Hand, usize)> = vec![];
    let mut two_pair: Vec<(String, Hand, usize)> = vec![];
    let mut three_of_kind: Vec<(String, Hand, usize)> = vec![];
    let mut full_house: Vec<(String, Hand, usize)> = vec![];
    let mut four_of_kind: Vec<(String, Hand, usize)> = vec![];
    let mut five_of_a_kind: Vec<(String, Hand, usize)> = vec![];
    let mut games: Vec<(String, Hand, usize)> = vec![];
    let lines = reader.lines().map(|l| l.unwrap());

    for line in lines {
        let v = line.trim().split_ascii_whitespace().collect::<Vec<&str>>();
        let hand = v[0].trim().to_string();
        let bid = v[1].trim().parse::<usize>().unwrap();
        let hand_type = get_hand(&hand);
        match hand_type {
            Hand::FiveOfAKind => five_of_a_kind.push((hand, hand_type, bid)),
            Hand::FourOfAKind => four_of_kind.push((hand, hand_type, bid)),
            Hand::FullHouse => full_house.push((hand, hand_type, bid)),
            Hand::ThreeOfAKind => three_of_kind.push((hand, hand_type, bid)),
            Hand::TwoPair => two_pair.push((hand, hand_type, bid)),
            Hand::OnePair => one_pair.push((hand, hand_type, bid)),
            Hand::HighCard => high_card.push((hand, hand_type, bid)),
        }
    }
    let _max_score = high_card.len()
        + one_pair.len()
        + two_pair.len()
        + three_of_kind.len()
        + full_house.len()
        + four_of_kind.len()
        + five_of_a_kind.len();
    high_card.sort_by(|a, b| compare_hands(&b.0, &a.0));
    one_pair.sort_by(|a, b| compare_hands(&b.0, &a.0));
    two_pair.sort_by(|a, b| compare_hands(&b.0, &a.0));
    three_of_kind.sort_by(|a, b| compare_hands(&b.0, &a.0));
    full_house.sort_by(|a, b| compare_hands(&b.0, &a.0));
    four_of_kind.sort_by(|a, b| compare_hands(&b.0, &a.0));
    five_of_a_kind.sort_by(|a, b| compare_hands(&b.0, &a.0));
    games.append(&mut high_card);
    games.append(&mut one_pair);
    games.append(&mut two_pair);
    games.append(&mut three_of_kind);
    games.append(&mut full_house);
    games.append(&mut four_of_kind);
    games.append(&mut five_of_a_kind);
    let mut sum = 0;
    for (index, game) in games.iter().enumerate() {
        sum += (index + 1) * game.2;
    }
    println!("{sum}");
}

// fn get_hand_score(hand: &Hand) -> usize {
//     for (i, h) in HAND_SCORE.iter().enumerate() {
//         if h == hand {
//             return i;
//         }
//     }
//     0
// }

fn compare_hands(b: &str, a: &str) -> Ordering {
    for (x, y) in a.chars().zip(b.chars()) {
        let i = get_card_score(&x).cmp(&get_card_score(&y));
        if i != Ordering::Equal {
            return i;
        }
    }
    Ordering::Equal
}

#[allow(dead_code)]
fn get_hand(hand: &str) -> Hand {
    let mut my_map = HashMap::new();
    for letter in hand.chars() {
        let count = my_map.entry(letter).or_insert(0);
        *count += 1;
    }
    let value = my_map.values().max().unwrap();
    if my_map.len() == 1 {
        return Hand::FiveOfAKind;
    } else if my_map.len() == 2 {
        if value == &4 {
            return Hand::FourOfAKind;
        } else if value == &3 {
            return Hand::FullHouse;
        }
    } else if my_map.len() == 3 {
        if value == &3 {
            return Hand::ThreeOfAKind;
        } else if value == &2 {
            return Hand::TwoPair;
        }
    } else if my_map.len() == 4 {
        return Hand::OnePair;
    }
    Hand::HighCard
}

fn get_card_score(card: &char) -> usize {
    for (index, letter) in CARD_SCORE.iter().enumerate() {
        if letter == card {
            return index;
        }
    }
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_hand() {
        let hand = "323TK";
        let a = get_hand(hand);
        assert_eq!(a, Hand::OnePair);
        let hand = "AAAAA";
        let a = get_hand(hand);
        assert_eq!(a, Hand::FiveOfAKind);
        let hand = "AKAAA";
        let a = get_hand(hand);
        assert_eq!(a, Hand::FourOfAKind);
        let hand = "AKKAA";
        let a = get_hand(hand);
        assert_eq!(a, Hand::FullHouse);
        let hand = "AKQAA";
        let a = get_hand(hand);
        assert_eq!(a, Hand::ThreeOfAKind);
        let hand = "AKQAK";
        let a = get_hand(hand);
        assert_eq!(a, Hand::TwoPair);
        let hand = "AKQAT";
        let a = get_hand(hand);
        assert_eq!(a, Hand::OnePair);
        let hand = "AKQ9T";
        let a = get_hand(hand);
        assert_eq!(a, Hand::HighCard);
    }
}
