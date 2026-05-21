// 3043. Find the Length of the Longest Common Prefix
// https://leetcode.com/problems/find-the-length-of-the-longest-common-prefix/

struct Solution;

impl Solution {
    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_length_of_the_longest_common_prefix::Solution;

    #[test]
    fn test_longest_common_prefix_1() {
        let arr1 = [1, 10, 100].to_vec();
        let arr2 = [1000].to_vec();
        assert_eq!(3, Solution::longest_common_prefix(arr1, arr2));
    }

    #[test]
    fn test_longest_common_prefix_2() {
        let arr1 = [1, 2, 3].to_vec();
        let arr2 = [4, 4, 4].to_vec();
        assert_eq!(0, Solution::longest_common_prefix(arr1, arr2));
    }
}
