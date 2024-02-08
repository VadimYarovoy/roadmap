use std::collections::HashMap;

pub fn _is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map: HashMap<char, i32> = HashMap::new();

    for (sc, tc) in s.chars().zip(t.chars()) {
        *map.entry(sc).or_default() += 1;
        *map.entry(tc).or_default() -= 1;
    }

    map.into_values().all(|c| c == 0)
}

#[cfg(test)]
mod test_is_anagram {
    use super::*;

    #[test]
    fn test_not_anagram() {
        assert_eq!(_is_anagram("cat".to_string(), "tas".to_string()), false)
    }

    #[test]
    fn test_anagram() {
        assert_eq!(_is_anagram("cat".to_string(), "tac".to_string()), true)
    }
}
