// 2598. Smallest Missing Non-negative Integer After Operations
// https://leetcode.com/problems/smallest-missing-non-negative-integer-after-operations/

struct Solution;

impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        println!("nums: {:?}, value: {}", &nums, &value);

        // Count the frequency of each remainder (mod value)
        let mut remainder_count = vec![0; value as usize];

        for &num in &nums {
            // Handle negative numbers correctly with Euclidean modulo
            let remainder = ((num % value) + value) % value;
            remainder_count[remainder as usize] += 1;
        }

        // Try to form 0, 1, 2, 3, ... in order
        // For each number k, we need a number with remainder (k % value)
        let mut k = 0;
        loop {
            let needed_remainder = (k % value) as usize;

            if remainder_count[needed_remainder] == 0 {
                // Can't form k
                return k;
            }

            // Use one number with this remainder to form k
            remainder_count[needed_remainder] -= 1;
            k += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_missing_non_negative_integer_after_operations::Solution;

    #[test]
    fn test_find_smallest_integer_1() {
        let nums = [1, -10, 7, 13, 6, 8].to_vec();
        let value = 5;
        assert_eq!(4, Solution::find_smallest_integer(nums, value));
    }

    #[test]
    fn test_find_smallest_integer_2() {
        let nums = [1, -10, 7, 13, 6, 8].to_vec();
        let value = 7;
        assert_eq!(2, Solution::find_smallest_integer(nums, value));
    }
}
