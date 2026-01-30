use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut result: HashSet<&str> = HashSet::new();

    for candidate in possible_anagrams {
        if candidate.len() != word.len() || candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let original_candidate = candidate;
        let candidate = candidate.to_lowercase();
        let word = word.to_lowercase();

        let mut candidate_as_chars: Vec<char> = candidate.chars().collect();
        let mut word_as_chars: Vec<char> = word.chars().collect();

        candidate_as_chars.sort();
        word_as_chars.sort();

        if candidate_as_chars.iter().eq(word_as_chars.iter()) {
            result.insert(original_candidate);
        }
    }

    result
}
