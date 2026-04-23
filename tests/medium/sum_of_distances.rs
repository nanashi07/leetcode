// 2615. Sum of Distances
// https://leetcode.com/problems/sum-of-distances/

struct Solution;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sum_of_distances::Solution;

    #[test]
    fn test_distance_1() {
        let nums = [1, 3, 1, 1, 2].to_vec();
        assert_eq!([5, 0, 3, 4, 0].to_vec(), Solution::distance(nums));
    }

    #[test]
    fn test_distance_2() {
        let nums = [0, 5, 3].to_vec();
        assert_eq!([0, 0, 0].to_vec(), Solution::distance(nums));
    }
}
