// 2897. Apply Operations on Array to Maximize Sum of Squares
// https://leetcode.com/problems/apply-operations-on-array-to-maximize-sum-of-squares/

struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        const BIT_COUNT: usize = 31;

        let mut bit_counts = [0_i32; BIT_COUNT];
        for num in nums {
            let num = num as u32;
            for (bit, count) in bit_counts.iter_mut().enumerate() {
                *count += ((num >> bit) & 1) as i32;
            }
        }

        let mut answer = 0_i64;
        for _ in 0..k {
            let mut value = 0_i64;
            for (bit, count) in bit_counts.iter_mut().enumerate() {
                if *count > 0 {
                    value |= 1_i64 << bit;
                    *count -= 1;
                }
            }
            answer = (answer + value * value) % MOD;
        }

        answer as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::apply_operations_on_array_to_maximize_sum_of_squares::Solution;

    #[test]
    fn test_max_sum_1() {
        let nums = [2, 6, 5, 8].to_vec();
        let k = 2;
        assert_eq!(261, Solution::max_sum(nums, k));
    }

    #[test]
    fn test_max_sum_2() {
        let nums = [4, 5, 4, 7].to_vec();
        let k = 3;
        assert_eq!(90, Solution::max_sum(nums, k));
    }
}
