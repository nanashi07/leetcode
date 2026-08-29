// 2948. Make Lexicographically Smallest Array by Swapping Elements
// https://leetcode.com/problems/make-lexicographically-smallest-array-by-swapping-elements/

struct Solution;

impl Solution {
    pub fn lexicographically_smallest_array(nums: Vec<i32>, limit: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::make_lexicographically_smallest_array_by_swapping_elements::Solution;

    #[test]
    fn test_lexicographically_smallest_array_1() {
        let nums = [1, 5, 3, 9, 8].to_vec();
        let limit = 2;
        assert_eq!(
            [1, 3, 5, 8, 9].to_vec(),
            Solution::lexicographically_smallest_array(nums, limit)
        );
    }

    #[test]
    fn test_lexicographically_smallest_array_2() {
        let nums = [1, 7, 6, 18, 2, 1].to_vec();
        let limit = 3;
        assert_eq!(
            [1, 6, 7, 18, 1, 2].to_vec(),
            Solution::lexicographically_smallest_array(nums, limit)
        );
    }

    #[test]
    fn test_lexicographically_smallest_array_3() {
        let nums = [1, 7, 28, 19, 10].to_vec();
        let limit = 3;
        assert_eq!(
            [1, 7, 28, 19, 10].to_vec(),
            Solution::lexicographically_smallest_array(nums, limit)
        );
    }
}
