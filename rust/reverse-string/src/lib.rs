use unicode_reverse::reverse_grapheme_clusters_in_place;

pub fn reverse(input: &str) -> String {
    let mut reversed = input
        .split_ascii_whitespace()
        .map(|s| {
            let mut r = s.to_string();

            reverse_grapheme_clusters_in_place(&mut r);

            r
        })
        .collect::<Vec<String>>();

    reversed.reverse();

    reversed.join(" ")
}
