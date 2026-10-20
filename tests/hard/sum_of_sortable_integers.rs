// 3886. Sum of Sortable Integers
// https://leetcode.com/problems/sum-of-sortable-integers/

struct Solution;

impl Solution {
    pub fn sortable_integers(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::sum_of_sortable_integers::Solution;

    #[test]
    fn test_sortable_integers_1() {
        let nums = [3, 1, 2].to_vec();
        assert_eq!(3, Solution::sortable_integers(nums));
    }

    #[test]
    fn test_sortable_integers_2() {
        let nums = [7, 6, 5].to_vec();
        assert_eq!(0, Solution::sortable_integers(nums));
    }

    #[test]
    fn test_sortable_integers_3() {
        let nums = [5, 8].to_vec();
        assert_eq!(3, Solution::sortable_integers(nums));
    }
}
