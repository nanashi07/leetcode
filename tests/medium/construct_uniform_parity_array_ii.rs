// 3876. Construct Uniform Parity Array II
// https://leetcode.com/problems/construct-uniform-parity-array-ii/

struct Solution;

impl Solution {
    pub fn uniform_array(nums1: Vec<i32>) -> bool {
        let min_even = nums1.iter().filter(|&&x| x % 2 == 0).min();
        let min_odd = nums1.iter().filter(|&&x| x % 2 != 0).min();

        min_even.is_none() || min_odd.is_none() || min_odd.is_some_and(|odd| Some(odd) < min_even)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::construct_uniform_parity_array_ii::Solution;

    #[test]
    fn test_uniform_array_1() {
        let nums1 = [1, 4, 7].to_vec();
        assert!(Solution::uniform_array(nums1));
    }

    #[test]
    fn test_uniform_array_2() {
        let nums1 = [2, 3].to_vec();
        assert!(!Solution::uniform_array(nums1));
    }

    #[test]
    fn test_uniform_array_3() {
        let nums1 = [4, 6].to_vec();
        assert!(Solution::uniform_array(nums1));
    }

    #[test]
    fn test_uniform_array_4() {
        let nums1 = [11, 16].to_vec();
        assert!(Solution::uniform_array(nums1));
    }

    #[test]
    fn test_uniform_array_5() {
        let nums1 = [13, 10].to_vec();
        assert!(!Solution::uniform_array(nums1));
    }
}
