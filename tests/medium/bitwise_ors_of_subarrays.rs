// # 898. Bitwise ORs of Subarrays
// https://leetcode.com/problems/bitwise-ors-of-subarrays/

struct Solution;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::bitwise_ors_of_subarrays::Solution;

    #[test]
    fn test_subarray_bitwise_o_rs_1() {
        let arr = [0].to_vec();
        assert_eq!(1, Solution::subarray_bitwise_o_rs(arr));
    }

    #[test]
    fn test_subarray_bitwise_o_rs_2() {
        let arr = [1, 1, 2].to_vec();
        assert_eq!(3, Solution::subarray_bitwise_o_rs(arr));
    }

    #[test]
    fn test_subarray_bitwise_o_rs_3() {
        let arr = [1, 2, 4].to_vec();
        assert_eq!(6, Solution::subarray_bitwise_o_rs(arr));
    }
}
