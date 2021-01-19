use std::collections::{HashSet, HashMap};

fn count_chars(word: &str) -> HashMap<char, i32> {
    let mut count_of = HashMap::new();
    for c in word.to_lowercase().chars() {
        *count_of.entry(c).or_insert(0) += 1;
    }
    count_of
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let count_of = count_chars(word);
    let mut anagrams: HashSet<&'a str> = HashSet::new();

    for pa in possible_anagrams {
        if pa.len() != word.len() || pa.to_lowercase() == word.to_lowercase() {
            continue;
        }

        if count_chars(&pa) == count_of {
            anagrams.insert(&pa);
        }
    }

    anagrams
}
