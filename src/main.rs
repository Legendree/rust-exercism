use std::collections::HashSet;
use time::{Duration, PrimitiveDateTime as DateTime};

use crate::{clock::Clock, planet::Planet};

pub mod clock;
pub mod luhn;
pub mod minesweeper;
pub mod planet;
pub mod poker;
pub mod sublist;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut correct_anagrams: HashSet<&str> = HashSet::new();

    let mut word_bytes: Vec<char> = word.to_lowercase().chars().collect();
    word_bytes.sort_unstable();

    for anagram in possible_anagrams {
        let mut anagram_bytes: Vec<char> = anagram.to_lowercase().chars().collect();
        anagram_bytes.sort_unstable();

        if word.to_lowercase() != *anagram.to_lowercase() && anagram_bytes == word_bytes {
            correct_anagrams.insert(anagram);
        }
    }

    return correct_anagrams;
}

pub fn reverse(input: &str) -> String {
    let chars: Vec<char> = input.chars().rev().collect();
    let mut reversed = String::new();

    for c in chars {
        reversed += &c.to_string();
    }

    return reversed;
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::new(10 ^ 9, 0)
}

fn main() {
    // println!("Hello, world!");
    // let anagrams = anagrams_for("dick", &["ickd", "dick", "fickd", "retard", "faggot"]);

    // println!("anagrams are for dick are: {:?}", anagrams);

    // println!("{}", reverse("Denis"));

    // let time: Clock = Clock::new(0, 3).add_minutes(-4);

    // println!("{}", time);

    // let duration = planet::Duration::from(2_134_835_688);

    // let in_jupiter_years = planet::Mercury::years_during(&duration);

    // println!(
    //     "The duration is {:?} in jupiter years that would be {}",
    //     duration, in_jupiter_years
    // );

    //    let v1: Vec<u64> = (10..1_000_001).collect();
    //    let v2: Vec<u64> = (1..1_000_000).collect();

    // let type_of_sublist = sublist::sublist(&v1, &v2);

    //    println!("{:?}", type_of_sublist);

    let mines = [" * * ", "  *  ", "  *  ", "     "];

    let minesweeper = minesweeper::annotate(&mines);

    for mine_row in minesweeper {
        println!("{:?}", mine_row);
    }

    // let mines = [" * * ", "  *  ", "  *  ", "     "];
    //
    // let mut mine_mat: Vec<Vec<&str>> = Vec::new();
    //
    // for row in mines {
    //     let splitted: Vec<&str> = row.split("").collect();
    //     mine_mat.push(splitted);
    // }
    //
    // let mut mines: Vec<String> = Vec::new();
    //
    // for mat_row in mine_mat {
    //     let joined_str = mat_row.join("");
    //     mines.push(joined_str);
    // }
    //
    // println!("{:?}", mines);

    let credit_card = "059";

    let is_credit_valid = luhn::is_valid(credit_card);

    println!("{}", is_credit_valid);

    poker::winning_hands(&["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"]);
}

//          · * · * ·
//          · · * · ·
//          · · * · ·
//          · · · · ·
