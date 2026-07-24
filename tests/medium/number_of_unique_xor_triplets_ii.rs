// 3514. Number of Unique XOR Triplets II
// https://leetcode.com/problems/number-of-unique-xor-triplets-ii/

struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_unique_xor_triplets_ii::Solution;

    #[test]
    fn test_unique_xor_triplets_1() {
        let nums = [1, 3].to_vec();
        assert_eq!(2, Solution::unique_xor_triplets(nums));
    }

    #[test]
    fn test_unique_xor_triplets_2() {
        let nums = [6, 7, 8, 9].to_vec();
        assert_eq!(4, Solution::unique_xor_triplets(nums));
    }
}
