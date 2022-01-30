use std::collections::HashSet;
use time::{Duration, PrimitiveDateTime as DateTime};

use crate::clock::Clock;

pub mod clock;

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
    println!("Hello, world!");
    let anagrams = anagrams_for("dick", &["ickd", "dick", "fickd", "retard", "faggot"]);

    println!("anagrams are for dick are: {:?}", anagrams);

    println!("{}", reverse("Denis"));

    let time: Clock = Clock::new(3, -20);

    println!("{}", time);
}
