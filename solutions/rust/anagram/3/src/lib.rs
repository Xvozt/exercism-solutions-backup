use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    // prepare the word
    let word = word.to_lowercase();
    let mut word_as_chars: Vec<_> = word.chars().collect();
    word_as_chars.sort();

    // // create hashset for result
    // let mut result: HashSet<&str> = HashSet::new();

    // // find anagrams and add to the result set
    // for candidate in possible_anagrams {
    //     if candidate.len() != word.len() {
    //         continue;
    //     }
    //     let original_candidate = candidate;
    //     let candidate = candidate.to_lowercase();

    //     if candidate == word {
    //         continue;
    //     }

    //     let mut candidate_as_chars: Vec<char> = candidate.chars().collect();

    //     candidate_as_chars.sort();

    //     if candidate_as_chars == word_as_chars{
    //         result.insert(original_candidate);
    //     }
    // }

    // result

    possible_anagrams
        .iter()
        .filter_map(|candidate| {
            if candidate.len() != word.len() {
                return None;
            }
            let original_candidate = candidate;
            let candidate = candidate.to_lowercase();

            if candidate == word {
                return None;
            }

            let mut candidate_as_chars: Vec<char> = candidate.chars().collect();
            candidate_as_chars.sort();

            // if candidate_as_chars == word_as_chars {
            //     return Some(*original_candidate);
            // }
            // return None;

            // simplified version
            (candidate_as_chars == word_as_chars).then_some(*original_candidate)
        })
        .collect()
}
