// 3069. Distribute Elements Into Two Arrays I
// https://leetcode.com/problems/distribute-elements-into-two-arrays-i/

struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::distribute_elements_into_two_arrays_i::Solution;

    #[test]
    fn test_result_array_1() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!([2, 3, 1].to_vec(), Solution::result_array(nums));
    }

    #[test]
    fn test_result_array_2() {
        let nums = [5, 4, 3, 8].to_vec();
        assert_eq!([5, 3, 4, 8].to_vec(), Solution::result_array(nums));
    }
}
