// 2154. Keep Multiplying Found Values by Two
// https://leetcode.com/problems/keep-multiplying-found-values-by-two/

struct Solution;

impl Solution {
    pub fn find_final_value(nums: Vec<i32>, original: i32) -> i32 {
        println!("nums: {:?}, original: {original}", &nums);

        let mut original = original;
        let mut nums = nums;
        nums.sort_unstable();

        for n in &nums {
            if *n == original {
                original = 2 * original;
            }
        }

        original
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::keep_multiplying_found_values_by_two::Solution;

    #[test]
    fn test_find_final_value_1() {
        let nums = [5, 3, 6, 1, 12].to_vec();
        let original = 3;
        assert_eq!(24, Solution::find_final_value(nums, original));
    }

    #[test]
    fn test_find_final_value_2() {
        let nums = [2, 7, 9].to_vec();
        let original = 4;
        assert_eq!(4, Solution::find_final_value(nums, original));
    }
}
