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

    fn lt(&self, other: &Self) -> bool {
        let rank_parsed = self.rank;
        let other_rank_parsed = other.rank;
        rank_parsed.lt(&other_rank_parsed)
    }

    fn le(&self, other: &Self) -> bool {
        let rank_parsed = self.rank;
        let other_rank_parsed = other.rank;
        rank_parsed.le(&other_rank_parsed)
    }

    fn gt(&self, other: &Self) -> bool {
        let rank_parsed = self.rank;
        let other_rank_parsed = other.rank;
        rank_parsed.gt(&other_rank_parsed)
    }

    fn ge(&self, other: &Self) -> bool {
        let rank_parsed = self.rank;
        let other_rank_parsed = other.rank;
        rank_parsed.ge(&other_rank_parsed)
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
        let cards = infer_hand_type(hand);
        let hand_type = get_hand_type(&cards, hand);
        let hand_rank = get_hand_rank(&hand_type, hand);
        let hands_vec = hand_map.entry(hand_rank).or_insert(vec![]);
        hands_vec.push(hand);
    }

    let mut strongest_rank = 0;

    for rank in hand_map.keys() {
        if *rank > strongest_rank {
            strongest_rank = *rank;
        }
    }

    let hands = hand_map.get(&strongest_rank).unwrap().to_vec();

    let mut strongest_hands: Vec<&str> = Vec::new();
    let mut highest = 0;

    for hand in hands {
        let card_rank = get_hand_power(hand);

        if card_rank > highest {
            highest = card_rank;
            strongest_hands = vec![hand];
        } else if card_rank == highest {
            if get_card_sum(hand) < get_card_sum(strongest_hands[0]) {
                strongest_hands = vec![strongest_hands[0]]
            } else {
                strongest_hands.push(hand)
            }
        }
    }

    return strongest_hands;
}

fn get_hand_power<'a>(hand: &'a str) -> i8 {
    let cards = infer_hand_type(hand);
    let hand_type = get_hand_type(&cards, hand);

    let first_result = (cards[0].rank - cards[1].rank).abs();
    let second_result = (cards[1].rank - cards[2].rank).abs();
    let third_result = (cards[2].rank - cards[3].rank).abs();
    let fourth_result = (cards[3].rank - cards[4].rank).abs();

    if hand_type == HandType::TwoPair(hand) {
        if first_result == 0 && second_result != 0 && third_result == 0 && fourth_result != 0 {
            return cards[0].rank + cards[1].rank + cards[2].rank + cards[3].rank;
        } else if first_result != 0 && second_result == 0 && third_result != 0 && fourth_result == 0
        {
            return cards[2].rank + cards[3].rank + cards[3].rank + cards[4].rank;
        } else if first_result == 0 && second_result != 0 && third_result != 0 && fourth_result == 0
        {
            return cards[0].rank + cards[1].rank + cards[3].rank + cards[4].rank;
        }
    }

    get_card_sum(hand)
}

fn infer_hand_type<'a>(original_hand: &'a str) -> Vec<Card> {
    let cards: Vec<&'a str> = original_hand.split(" ").collect();

    let mut hand: Vec<Card> = Vec::new();

    for card in cards {
        let mut splitted_card: Vec<&str> = card.split("").collect();
        splitted_card = splitted_card[1..splitted_card.len() - 1].to_vec();

        let card = if splitted_card.len() > 2 {
            (
                format!("{}{}", splitted_card[0], splitted_card[1]),
                splitted_card[2],
            )
        } else {
            (splitted_card[0].to_string(), splitted_card[1])
        };

        let card = Card {
            rank: get_rank_by_char(&card.0, original_hand),
            symbol: card.1,
        };

        hand.push(card);
    }

    hand.sort();
    hand.reverse();

    hand
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
    } else if (first_result == 0 && second_result == 0 && third_result == 0)
        || (second_result == 0 && third_result == 0 && fourth_result == 0)
    {
        HandType::FourOfKind(original_hand)
    } else if (first_result == 0 && second_result == 0 && third_result != 0 && fourth_result == 0)
        || (fourth_result == 0 && third_result == 0 && second_result != 0 && first_result == 0)
    {
        HandType::FullHouse(original_hand)
    } else if is_hand_same_symbol(hand) {
        HandType::Flush(original_hand)
    } else if first_result == second_result
        && second_result == third_result
        && third_result == fourth_result
    {
        HandType::Straight(original_hand)
    } else if (first_result == 0 && second_result == 0 && third_result != 0 && fourth_result != 0)
        || (first_result != 0 && second_result != 0 && third_result == 0 && fourth_result == 0)
    {
        HandType::ThreeOfKind(original_hand)
    } else if (first_result == 0 && second_result != 0 && third_result == 0 && fourth_result != 0)
        || (first_result != 0 && second_result == 0 && third_result != 0 && fourth_result == 0)
        || (first_result == 0 && second_result != 0 && third_result != 0 && fourth_result == 0)
    {
        HandType::TwoPair(original_hand)
    } else if first_result == 0 || second_result == 0 || third_result == 0 || fourth_result == 0 {
        HandType::OnePair(original_hand)
    } else {
        HandType::HighCard(original_hand)
    }
}

fn get_rank_by_char<'a>(c: &'a str, hand: &'a str) -> i8 {
    if c == "J" {
        11
    } else if c == "Q" {
        12
    } else if c == "K" {
        13
    } else if c == "A" {
        is_small_or_big(c, hand, 14, 1)
    } else {
        c.parse::<i8>().unwrap()
    }
}

fn is_small_or_big<'a>(symbol: &str, hand: &'a str, high: i8, low: i8) -> i8 {
    let cards: Vec<&str> = hand.split(" ").collect();

    let mut ace = false;

    for card in cards {
        let mut splitted_card: Vec<&str> = card.split("").collect();
        splitted_card = splitted_card[1..splitted_card.len() - 1].to_vec();

        let card = if splitted_card.len() > 2 {
            (
                format!("{}{}", splitted_card[0], splitted_card[1]),
                splitted_card[2],
            )
        } else {
            (splitted_card[0].to_string(), splitted_card[1])
        };

        if card.0 == symbol {
            ace = true;
        }

        let parsed_card_number = match card.0.parse::<i8>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if ace && parsed_card_number == 2 {
            return low;
        }
    }

    high
}

fn is_hand_same_symbol(hand: &Vec<Card>) -> bool {
    hand[0].symbol == hand[1].symbol
        && hand[1].symbol == hand[2].symbol
        && hand[2].symbol == hand[3].symbol
        && hand[3].symbol == hand[4].symbol
}

fn get_hand_rank<'a>(hand_type: &HandType, hand: &'a str) -> i8 {
    if *hand_type == HandType::FiveOfKind(hand) {
        return 11;
    } else if *hand_type == HandType::StraightFlush(hand) {
        return 10;
    } else if *hand_type == HandType::FourOfKind(hand) {
        return 9;
    } else if *hand_type == HandType::FullHouse(hand) {
        return 8;
    } else if *hand_type == HandType::Flush(hand) {
        return 7;
    } else if *hand_type == HandType::Straight(hand) {
        return 6;
    } else if *hand_type == HandType::ThreeOfKind(hand) {
        return 5;
    } else if *hand_type == HandType::TwoPair(hand) {
        return 4;
    } else if *hand_type == HandType::OnePair(hand) {
        return 3;
    }

    0
}

fn get_card_sum<'a>(hand: &'a str) -> i8 {
    let cards: Vec<&str> = hand.split(" ").collect();

    let mut total_sum: i8 = 0;

    for card in cards {
        let card_split: Vec<&str> = card.split("").collect();
        total_sum += get_rank_by_char(card_split[1], hand);
    }

    total_sum
}
