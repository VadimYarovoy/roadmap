use std::{collections::HashSet, iter::FromIterator};

fn _longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = HashSet::from_iter(nums.into_iter());
    let mut max = 0;

    for n in &set {
        if !set.contains(&(n-1)) {
            let mut next = n + 1;
            let mut cnt = 1;

            while set.contains(&next) {
                cnt += 1;
                next += 1;
            }

            max = max.max(cnt);
        }
    }

    max
}

#[cfg(test)]
mod test_longest_consecutive {
    use super::*;

    #[test]
    fn test_one() {
        assert_eq!(_longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    }

    #[test]
    fn test_two() {
        assert_eq!(_longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9);
    }
}
