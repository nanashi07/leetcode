// # 2411. Smallest Subarrays With Maximum Bitwise OR
// https://leetcode.com/problems/smallest-subarrays-with-maximum-bitwise-or/

struct Solution;

impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::medium::smallest_subarrays_with_maximum_bitwise_or::Solution;

    #[test]
    fn test_smallest_subarrays_1() {
        let nums = [1, 0, 2, 1, 3].to_vec();
        assert_eq!([3, 3, 2, 2, 1].to_vec(), Solution::smallest_subarrays(nums));
    }

    #[test]
    fn test_smallest_subarrays_2() {
        let nums = [1, 2].to_vec();
        assert_eq!([2, 1].to_vec(), Solution::smallest_subarrays(nums));
    }
}
