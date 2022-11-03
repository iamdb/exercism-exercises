use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut set = HashSet::new();

    for p in possible_anagrams {
        if is_anagram(word, p) {
            set.insert(*p);
        }
    }

    set
}

fn is_anagram(test_word: &str, possible_anagram: &str) -> bool {
    if test_word.len() != possible_anagram.len() {
        return false;
    }

    let test_word = test_word.to_lowercase();
    let possible_anagram = possible_anagram.to_lowercase();

    if test_word == possible_anagram {
        return false;
    }

    let mut test_vec = test_word.chars().collect::<Vec<char>>();
    test_vec.sort_unstable();

    let mut possible_vec = possible_anagram.chars().collect::<Vec<char>>();
    possible_vec.sort_unstable();

    test_vec.eq(&possible_vec)
}
