use std::collections::{HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut anagrams_set = HashSet::new();
    let mut target_word_letters_count_hashmap: HashMap<char, usize> = HashMap::new();
    let target_word = word.to_lowercase();
    for letter in target_word.chars() {
        let count = target_word_letters_count_hashmap.entry(letter).or_insert(0);
        *count += 1;
    }
    
    for possible_anagram_word in possible_anagrams {
        if target_word != possible_anagram_word.to_lowercase() {
            let mut possible_anagram_word_letters_count_hashmap: HashMap<char, usize> = HashMap::new();
            for letter in possible_anagram_word.to_lowercase().chars() {
                let count = possible_anagram_word_letters_count_hashmap.entry(letter).or_insert(0);
                *count += 1;
            }
            if target_word_letters_count_hashmap == possible_anagram_word_letters_count_hashmap {
                anagrams_set.insert(*possible_anagram_word);
            }
        }
    }
    anagrams_set
}
