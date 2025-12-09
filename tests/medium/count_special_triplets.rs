// 3583. Count Special Triplets
// https://leetcode.com/problems/count-special-triplets/

struct Solution;

impl Solution {
    pub fn special_triplets(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_special_triplets::Solution;

    #[test]
    fn test_special_triplets_1() {
        let nums = [6, 3, 6].to_vec();
        assert_eq!(1, Solution::special_triplets(nums));
    }

    #[test]
    fn test_special_triplets_2() {
        let nums = [0, 1, 0, 0].to_vec();
        assert_eq!(1, Solution::special_triplets(nums));
    }

    #[test]
    fn test_special_triplets_3() {
        let nums = [8, 4, 2, 8, 4].to_vec();
        assert_eq!(2, Solution::special_triplets(nums));
    }
}
