// 3637. Trionic Array I
// https://leetcode.com/problems/trionic-array-i/

struct Solution;

impl Solution {
    pub fn is_trionic(nums: Vec<i32>) -> bool {
        println!("nums: {nums:?}");

        let mut c = 0;
        for i in 1..nums.len() {
            let d = nums[i] - nums[i - 1];
            if d == 0 {
                return false;
            } else {
                if i == 1 {
                    if d < 0 {
                        return false;
                    }
                } else {
                    if d * (nums[i - 1] - nums[i - 2]) < 0 {
                        c = c + 1;
                    }
                }
            }
        }

        c == 2
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::trionic_array_i::Solution;

    #[test]
    fn test_is_trionic_1() {
        let nums = [1, 3, 5, 4, 2, 6].to_vec();
        assert_eq!(true, Solution::is_trionic(nums));
    }

    #[test]
    fn test_is_trionic_2() {
        let nums = [2, 1, 3].to_vec();
        assert_eq!(false, Solution::is_trionic(nums));
    }

    #[test]
    fn test_is_trionic_3() {
        let nums = [7, 7, 1, 6, 5].to_vec();
        assert_eq!(false, Solution::is_trionic(nums));
    }
}
