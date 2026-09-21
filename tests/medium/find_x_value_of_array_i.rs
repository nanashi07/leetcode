// 3524. Find X Value of Array I
// https://leetcode.com/problems/find-x-value-of-array-i/

struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_x_value_of_array_i::Solution;

    #[test]
    fn test_result_array_1() {
        let nums = [1, 2, 3, 4, 5].to_vec();
        let k = 3;
        assert_eq!([9, 2, 4].to_vec(), Solution::result_array(nums, k));
    }

    #[test]
    fn test_result_array_2() {
        let nums = [1, 2, 4, 8, 16, 32].to_vec();
        let k = 4;
        assert_eq!([18, 1, 2, 0].to_vec(), Solution::result_array(nums, k));
    }

    #[test]
    fn test_result_array_3() {
        let nums = [1, 1, 2, 1, 1].to_vec();
        let k = 2;
        assert_eq!([9, 6].to_vec(), Solution::result_array(nums, k));
    }
}
