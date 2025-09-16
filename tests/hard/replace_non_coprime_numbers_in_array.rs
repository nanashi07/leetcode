// # 2197. Replace Non-Coprime Numbers in Array
// https://leetcode.com/problems/replace-non-coprime-numbers-in-array/

struct Solution;
impl Solution {
    pub fn replace_non_coprimes(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::replace_non_coprime_numbers_in_array::Solution;

    #[test]
    fn test_replace_non_coprimes_1() {
        let nums = [6, 4, 3, 2, 7, 6, 2].to_vec();
        assert_eq!([12, 7, 6].to_vec(), Solution::replace_non_coprimes(nums));
    }

    #[test]
    fn test_replace_non_coprimes_2() {
        let nums = [2, 2, 1, 1, 3, 3, 3].to_vec();
        assert_eq!([2, 1, 1, 3].to_vec(), Solution::replace_non_coprimes(nums));
    }
}
