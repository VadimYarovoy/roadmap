use std::collections::HashMap;

fn _two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut vals: HashMap<i32, usize> = HashMap::new();

    for (i, v) in nums.iter().enumerate() {
        let goal = target - v;
        if vals.contains_key(&goal) {
            return vec![vals[&goal] as i32, i as i32];
        } else {
            vals.insert(*v, i);
        }
    }

    vec![]
}

#[cfg(test)]
mod test_two_sum {
    use super::*;

    #[test]
    fn test_with_sum() {
        assert_eq!(_two_sum(vec![1, 2, 3, 4, 5], 8), vec![2, 4]);
    }
}
