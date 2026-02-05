// 3379. Transformed Array
// https://leetcode.com/problems/transformed-array/

struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        println!("nums: {nums:?}");

        let len = nums.len() as i32;
        let mut results = vec![0; len as usize];

        for i in 0..len {
            let n = &nums[i as usize];
            let mut n = i + *n;
            if n < 0 {
                n = n + (n.abs() as f64 / len as f64).ceil() as i32 * len;
            }
            n = n % len;
            results[i as usize] = nums[n as usize];
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::transformed_array::Solution;

    #[test]
    fn test_construct_transformed_array_1() {
        let nums = [3, -2, 1, 1].to_vec();
        let output = [1, 1, 1, 3].to_vec();
        assert_eq!(output, Solution::construct_transformed_array(nums));
    }

    #[test]
    fn test_construct_transformed_array_2() {
        let nums = [-1, 4, -1].to_vec();
        let output = [-1, -1, 4].to_vec();
        assert_eq!(output, Solution::construct_transformed_array(nums));
    }

    #[test]
    fn test_construct_transformed_array_3() {
        let nums = [-10].to_vec();
        let output = [-10].to_vec();
        assert_eq!(output, Solution::construct_transformed_array(nums));
    }
}
