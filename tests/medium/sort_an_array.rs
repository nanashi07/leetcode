// # 912. Sort an Array
// https://leetcode.com/problems/sort-an-array/

struct Solution;

impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sort_array_1() {
        let nums = vec![5, 2, 3, 1];
        let expected = vec![1, 2, 3, 5];
        assert_eq!(expected, nums);
    }

    #[test]
    fn test_sort_array_2() {
        let nums = vec![5, 1, 1, 2, 0, 0];
        let expected = vec![0, 0, 1, 1, 2, 5];
        assert_eq!(expected, nums);
    }
}
