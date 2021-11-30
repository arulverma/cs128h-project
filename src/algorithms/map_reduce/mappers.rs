use std::collections::HashMap;

/// @param `words` - a `Vec` of `String`s. You can assume that all `String`s 
///                  have been formatted and stripped of whitespace, 
///                  punctutation, etc.
/// 
/// @return a `HashMap` with distinct words as keys and the number of 
///         occurrences of the key in given `Vec` of `String`s.
pub fn word_count_mapper(words: &Vec<String>) -> HashMap<String, usize> {
    let mut map: HashMap<String, usize> = HashMap::new();

    for word in words {
        let word_string = word.to_string();
        if map.contains_key(word) {
            map.insert(word_string, map.get(word).unwrap() + (1 as usize));
            continue;
        }
        map.insert(word_string, 1);
    }

    map
}

/// @param `words` - a `Vec` of `String`s. You can assume that all `String`s 
///                  have been formatted and stripped of whitespace, 
///                  punctutation, etc.
/// 
/// @return a `HashMap` with distinct word *LENGTHS* as keys and the number of 
///         occurrences of the key in given `Vec` of `String`s.
pub fn word_length_mapper(words: &Vec<String>) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for word in words {
        let len = word.len() as usize;
        if map.contains_key(&len) {
            map.insert(len, map.get(&len).unwrap() + (1 as usize));
            continue;
        }
        map.insert(len, 1);
    }

    map
}
