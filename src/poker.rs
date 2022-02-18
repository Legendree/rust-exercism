use std::{
    cmp::{Ordering, PartialEq},
    collections::HashMap,
};

#[derive(Debug)]
struct Card<'a> {
    rank: i8,
    symbol: &'a str,
}

impl<'a> PartialEq for Card<'a> {
    fn eq(&self, other: &Self) -> bool {
        let rank_parsed = self.rank;
        let other_rank_parsed = other.rank;
        rank_parsed.eq(&other_rank_parsed)
    }

    fn ne(&self, other: &Self) -> bool {
        let rank_parsed = self.rank;
        let other_rank_parsed = other.rank;
        rank_parsed.ne(&other_rank_parsed)
    }
}

impl<'a> PartialOrd for Card<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank_parsed = self.rank;
        let other_rank_parsed = other.rank;
        rank_parsed.partial_cmp(&other_rank_parsed)
    }
}

impl<'a> Eq for Card<'a> {}

impl<'a> Ord for Card<'a> {
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_parsed = self.rank;
        let other_rank_parsed = other.rank;
        rank_parsed.cmp(&other_rank_parsed)
    }
}

#[derive(PartialEq, Debug)]
enum HandType<'a> {
    FiveOfKind(&'a str),
    StraightFlush(&'a str),
    FourOfKind(&'a str),
    FullHouse(&'a str),
    Flush(&'a str),
    Straight(&'a str),
    ThreeOfKind(&'a str),
    TwoPair(&'a str),
    OnePair(&'a str),
    HighCard(&'a str),
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut hand_map: HashMap<i8, Vec<&'a str>> = HashMap::new();

    for hand in hands {
        let hand_type = infer_hand_type(*hand);
        // println!("{:?}", hand_type);
        let hand_rank = get_hand_rank(&hand_type, hand);
        let hands_vec = hand_map.entry(hand_rank).or_insert(vec![*hand]);
        hands_vec.push(*hand);
    }

    let mut strongest_rank = 0;

    for rank in hand_map.keys() {
        if *rank > strongest_rank {
            strongest_rank = *rank;
        }
    }

    println!("{:?}", hand_map);

    return hand_map.get(&strongest_rank).unwrap().to_vec();
}

fn infer_hand_type<'a>(original_hand: &'a str) -> HandType {
    let cards: Vec<&'a str> = original_hand.split(" ").collect();

    let mut hand: Vec<Card> = Vec::new();

    for card in cards {
        let mut splitted_card: Vec<&str> = card.split("").collect();
        splitted_card = splitted_card[1..splitted_card.len() - 1].to_vec();

        let card = Card {
            rank: get_rank_by_char(splitted_card[0]),
            symbol: splitted_card[1],
        };

        hand.push(card);
    }

    hand.sort();
    hand.reverse();

    // println!("{:?}", hand);

    get_hand_type(&hand, original_hand)
}

fn get_hand_type<'a>(hand: &Vec<Card>, original_hand: &'a str) -> HandType<'a> {
    let first_result = (hand[0].rank - hand[1].rank).abs();
    let second_result = (hand[1].rank - hand[2].rank).abs();
    let third_result = (hand[2].rank - hand[3].rank).abs();
    let fourth_result = (hand[3].rank - hand[4].rank).abs();

    if first_result == 0 && second_result == 0 && third_result == 0 && fourth_result == 0 {
        HandType::FiveOfKind(original_hand)
    } else if first_result == second_result
        && second_result == third_result
        && third_result == fourth_result
        && is_hand_same_symbol(hand)
    {
        HandType::StraightFlush(original_hand)
    } else if first_result == 0 && second_result == 0 && third_result == 0 {
        HandType::FourOfKind(original_hand)
    } else if first_result == 0 && second_result == 0 && third_result != 0 && fourth_result == 0 {
        HandType::FullHouse(original_hand)
    } else if first_result > second_result
        && second_result > third_result
        && third_result > fourth_result
        && is_hand_same_symbol(hand)
    {
        HandType::Flush(original_hand)
    } else if first_result == second_result
        && second_result == third_result
        && third_result == fourth_result
    {
        HandType::Straight(original_hand)
    } else if first_result == 0 && second_result == 0 && third_result != 0 && fourth_result != 0 {
        HandType::ThreeOfKind(original_hand)
    } else if first_result == 0 && second_result != 0 && third_result == 0 && fourth_result != 0 {
        HandType::TwoPair(original_hand)
    } else if first_result == 0 && second_result != 0 && third_result != 0 && fourth_result != 0 {
        HandType::OnePair(original_hand)
    } else {
        HandType::HighCard(original_hand)
    }
}

fn get_rank_by_char<'a>(c: &'a str) -> i8 {
    if c == "J" {
        11
    } else if c == "Q" {
        12
    } else if c == "K" {
        13
    } else if c == "A" {
        14
    } else {
        c.parse::<i8>().unwrap()
    }
}

fn is_hand_same_symbol(hand: &Vec<Card>) -> bool {
    hand[0].symbol == hand[1].symbol
        && hand[1].symbol == hand[2].symbol
        && hand[2].symbol == hand[3].symbol
        && hand[3].symbol == hand[4].symbol
}

fn get_hand_rank<'a>(hand_type: &HandType, hand: &'a str) -> i8 {
    if *hand_type == HandType::FiveOfKind(hand) {
        11;
    } else if *hand_type == HandType::StraightFlush(hand) {
        10;
    } else if *hand_type == HandType::FourOfKind(hand) {
        9;
    } else if *hand_type == HandType::FullHouse(hand) {
        8;
    } else if *hand_type == HandType::Flush(hand) {
        7;
    } else if *hand_type == HandType::Straight(hand) {
        6;
    } else if *hand_type == HandType::ThreeOfKind(hand) {
        5;
    } else if *hand_type == HandType::TwoPair(hand) {
        4;
    } else if *hand_type == HandType::OnePair(hand) {
        3;
    }

    0
}
