use std::collections::HashSet;

pub fn _contains_duplicate(nums: Vec<i32>) -> bool {
    let mut vals = HashSet::new();

    for i in nums {
        if !vals.insert(i) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod test_contains_duplicate {
    use super::*;

    #[test]
    fn test_with_no_dublicate() {
        let v = vec![1, 2, 3, 4, 5, 6];
        assert_eq!(_contains_duplicate(v), false);
    }

    #[test]
    fn test_with_dublicate() {
        let v = vec![1, 2, 2, 3, 5, 6];
        assert_eq!(_contains_duplicate(v), true);
    }
}
