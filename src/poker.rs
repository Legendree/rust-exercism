use std::{cmp::Ordering, collections::HashMap, ops::Sub};

#[derive(Debug)]
struct Card<'a> {
    rank: &'a str,
    symbol: &'a str,
}

impl<'a> PartialEq for Card<'a> {
    fn eq(&self, other: &Self) -> bool {
        let rank_parsed = get_rank(self);
        let other_rank_parsed = get_rank(other);
        rank_parsed.eq(&other_rank_parsed)
    }

    fn ne(&self, other: &Self) -> bool {
        let rank_parsed = get_rank(self);
        let other_rank_parsed = get_rank(other);
        rank_parsed.ne(&other_rank_parsed)
    }
}

impl<'a> PartialOrd for Card<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let rank_parsed = get_rank(self);
        let other_rank_parsed = get_rank(other);
        rank_parsed.partial_cmp(&other_rank_parsed)
    }
}

enum HandType<'a> {
    FIVE_OF_KIND(&'a str),
    STRAIGHT_FLUSH(&'a str),
    FOUR_OF_KIND(&'a str),
    FULL_HOUSE(&'a str),
    FLUSH(&'a str),
    STRAIGHT(&'a str),
    THREE_OF_KIND(&'a str),
    TWO_PAIR(&'a str),
    ONE_PAIR(&'a str),
    HIGH_CARD(&'a str),
}

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    for hand in hands {
        infer_hand_type(*hand);
    }

    return vec![""];
}

fn infer_hand_type<'a>(hand: &'a str) -> HandType {
    let cards: Vec<&'a str> = hand.split(" ").collect();

    let mut pairs: HashMap<&'a str, Vec<Card>> = HashMap::new();

    for card in cards {
        let mut splitted_card: Vec<&str> = card.split("").collect();
        splitted_card = splitted_card[1..splitted_card.len() - 1].to_vec();

        let card = Card {
            rank: splitted_card[0],
            symbol: splitted_card[1],
        };

        let existing_symbol = pairs.entry(splitted_card[0]).or_insert(vec![]);
        existing_symbol.push(card);
    }

    println!("{:?}", pairs);

    HandType::FIVE_OF_KIND(hand)
}

fn get_rank(card: &Card) -> u8 {
    match card.rank.parse::<u8>() {
        Ok(rank) => rank,
        Err(_) => get_rank_by_char(card.rank),
    }
}

fn get_rank_by_char<'a>(c: &'a str) -> u8 {
    if c == "J" {
        11
    } else if c == "Q" {
        12
    } else if c == "K" {
        13
    } else if c == "A" {
        14
    } else {
        0
    }
}
