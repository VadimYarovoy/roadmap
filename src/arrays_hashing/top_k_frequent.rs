use std::collections::HashMap;

pub fn _top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    for n in nums {
        *map.entry(n).or_insert(0) += 1;
    }

    let mut frec: Vec<(i32, i32)> = map.into_iter().collect();

    frec.sort_by_key(|t: &(i32, i32)| -t.1);

    frec.into_iter().take(k as usize).map(|(a, _)| a).collect()
}
