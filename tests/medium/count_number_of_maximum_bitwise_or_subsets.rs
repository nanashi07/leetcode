// # 2044. Count Number of Maximum Bitwise-OR Subsets
// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/

struct Solution;

impl Solution {
    // https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/editorial/
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let mut xor = 0;
        for n in &nums {
            xor = xor | *n;
        }
        println!("xor: {}", &xor);

        Self::count(&nums, 0, 0, xor)
    }

    fn count(nums: &Vec<i32>, index: usize, current_xor: i32, target_xor: i32) -> i32 {
        if index == nums.len() {
            return if current_xor == target_xor { 1 } else { 0 };
        }
        // next by filling with it or not
        let count_without = Self::count(nums, index + 1, current_xor, target_xor);
        let count_with = Self::count(nums, index + 1, current_xor | nums[index], target_xor);

        count_with + count_without
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_number_of_maximum_bitwise_or_subsets::Solution;

    #[test]
    fn test_count_max_or_subsets_1() {
        let nums = [3, 1].to_vec();
        assert_eq!(2, Solution::count_max_or_subsets(nums));
    }

    #[test]
    fn test_count_max_or_subsets_2() {
        let nums = [2, 2, 2].to_vec();
        assert_eq!(7, Solution::count_max_or_subsets(nums));
    }

    #[test]
    fn test_count_max_or_subsets_3() {
        let nums = [3, 2, 1, 5].to_vec();
        assert_eq!(6, Solution::count_max_or_subsets(nums));
    }
}
