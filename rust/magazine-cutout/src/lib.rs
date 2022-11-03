use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let magazine = magazine
        .iter()
        .filter(|m| note.contains(m))
        .cloned()
        .collect::<Vec<&str>>();

    let mag_counts = into_hashmap(&magazine);
    let note_counts = into_hashmap(note);

    if mag_counts.eq(&note_counts) {
        true
    } else {
        let mag_sum: usize = mag_counts.values().sum();
        let note_sum: usize = note_counts.values().sum();

        mag_sum > note_sum
    }
}

fn word_count(list: &[&str], word: &str) -> usize {
    list.iter().filter(|i| **i == word).count()
}

fn into_hashmap(list: &[&str]) -> HashMap<String, usize> {
    let mut map = HashMap::new();

    for i in list {
        map.insert(i.to_string(), word_count(list, i));
    }

    map
}
