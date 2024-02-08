pub fn _product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![1; nums.len()];

    for i in 1..nums.len() {
        res[i] = nums[i - 1] * res[i - 1];
    }

    let mut r = 1;

    for i in (0..nums.len()).rev() {
        res[i] *= r;
        r *= nums[i];
    }

    res
}

#[cfg(test)]
mod test_product_except_self {
    use super::*;

    #[test]
    fn test_how() {
        _product_except_self(vec![1, 2, 3, 4]);
    }
}
