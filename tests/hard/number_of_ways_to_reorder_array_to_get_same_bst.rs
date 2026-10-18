// 1569. Number of Ways to Reorder Array to Get Same BST
// https://leetcode.com/problems/number-of-ways-to-reorder-array-to-get-same-bst/

struct Solution;

impl Solution {
    pub fn num_of_ways(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_ways_to_reorder_array_to_get_same_bst::Solution;

    #[test]
    fn test_num_of_ways_1() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!(1, Solution::num_of_ways(nums));
    }

    #[test]
    fn test_num_of_ways_2() {
        let nums = [3, 4, 5, 1, 2].to_vec();
        assert_eq!(5, Solution::num_of_ways(nums));
    }

    #[test]
    fn test_num_of_ways_3() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(0, Solution::num_of_ways(nums));
    }
}
