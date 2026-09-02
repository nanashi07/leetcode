// 3875. Construct Uniform Parity Array I
// https://leetcode.com/problems/construct-uniform-parity-array-i/

struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::construct_uniform_parity_array_i::Solution;

    #[test]
    fn test_uniform_array_1() {
        let nums1 = [2, 3].to_vec();
        assert!(Solution::uniform_array(nums1));
    }

    #[test]
    fn test_uniform_array_2() {
        let nums1 = [4, 6].to_vec();
        assert!(Solution::uniform_array(nums1));
    }
}
