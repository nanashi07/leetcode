// 3525. Find X Value of Array II
// https://leetcode.com/problems/find-x-value-of-array-ii/

struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_x_value_of_array_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_result_array_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        let k = 3;
        let queries = to_vec2d([[2, 2, 0, 2], [3, 3, 3, 0], [0, 1, 0, 1]]);
        assert_eq!([2, 2, 2].to_vec(), Solution::result_array(nums, k, queries));
    }

    #[test]
    fn test_result_array_2() {
        let nums = [1, 2, 4, 8, 16, 32].to_vec();
        let k = 4;
        let queries = to_vec2d([[0, 2, 0, 2], [0, 2, 0, 1]]);
        assert_eq!([1, 0].to_vec(), Solution::result_array(nums, k, queries));
    }

    #[test]
    fn test_result_array_3() {
        let nums = [1, 1, 2, 1, 1].to_vec();
        let k = 2;
        let queries = to_vec2d([[2, 1, 0, 1]]);
        assert_eq!([5].to_vec(), Solution::result_array(nums, k, queries));
    }
}
