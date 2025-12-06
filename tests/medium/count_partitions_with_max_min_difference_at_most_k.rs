// 3578. Count Partitions With Max-Min Difference at Most K
// https://leetcode.com/problems/count-partitions-with-max-min-difference-at-most-k/

struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_partitions_with_max_min_difference_at_most_k::Solution;

    #[test]
    fn test_count_partitions_1() {
        let nums = [9, 4, 1, 3, 7].to_vec();
        let k = 4;
        assert_eq!(6, Solution::count_partitions(nums, k));
    }

    #[test]
    fn test_count_partitions_2() {
        let nums = [3, 3, 4].to_vec();
        let k = 0;
        assert_eq!(2, Solution::count_partitions(nums, k));
    }
}
