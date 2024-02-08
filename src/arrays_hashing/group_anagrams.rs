use std::collections::HashMap;

fn _group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<[u8; 26], Vec<String>> = HashMap::new();

    for s in strs {
        let mut key = [0_u8; 26];
        for ch in s.chars() {
            key[ch as usize - 'a' as usize] += 1;
        }

        if let Some(vals) = map.get_mut(&key) {
            vals.push(s);
        } else {
            map.insert(key, vec![s]);
        }
    }

    map.into_values().collect()
}

#[cfg(test)]
mod test_group_anagrams {
    use super::*;

    #[test]
    fn test_contains_anagrams() {
        let v = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        let res = vec![
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
        ];
        assert_eq!(_group_anagrams(v), res);
    }
}
