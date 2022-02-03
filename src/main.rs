use std::collections::HashSet;
use time::{Duration, PrimitiveDateTime as DateTime};

use crate::{clock::Clock, planet::Planet};

pub mod clock;
pub mod planet;
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

/// Minesweeper

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut minefield_mat: Vec<Vec<u8>> = Vec::new();

    for mine_row in minefield {
        let mines = mine_row.as_bytes();
        minefield_mat.push(mines.to_vec());
    }

    let mut mine_field = minefield_mat.clone();

    for (j, y) in minefield_mat.iter().enumerate() {
        for (i, _x) in y.iter().enumerate() {
            count_horizontal(&mut mine_field, j, i, 0);
        }
    }

    mine_field
        .iter()
        .map(|row| {
            let mut stringified_mine_row = String::new();
            row.iter()
                .for_each(|byte| stringified_mine_row.push(*byte as char));
            stringified_mine_row
        })
        .collect()
}

fn count_horizontal(mine_field: &mut Vec<Vec<u8>>, x: usize, y: usize, asterixes: u8) -> u8 {
    if is_pos_available(mine_field, y) && is_pos_available(&mine_field[y], x) {
        let mine = mine_field[y][x];
        let new_asterix_count = if contains_asterix(mine) {
            asterixes + 1
        } else {
            asterixes
        };
        return count_horizontal(mine_field, x + 1, y + 1, new_asterix_count);
    } else {
        mine_field[y - 1][x - 1] = format!("{}", asterixes).as_bytes()[0];
        return asterixes;
    }

    asterixes
}

fn contains_asterix(byte: u8) -> bool {
    byte.to_string() == "*".to_string()
}

fn is_pos_available<T>(slice: &[T], index: usize) -> bool {
    !(slice.len() <= index)
}

/// End of minesweeper

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

    let v1: Vec<u64> = (10..1_000_001).collect();
    let v2: Vec<u64> = (1..1_000_000).collect();

    let mines = ["1*22*1", "12*322", " 123*2", "112*4*", "1*22*2", "111111"];

    let type_of_sublist = sublist::sublist(&v1, &v2);

    println!("{:?}", type_of_sublist);

    annotate(&mines);
}
