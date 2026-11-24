// 3411. Maximum Subarray With Equal Products
// https://leetcode.com/problems/maximum-subarray-with-equal-products/

struct Solution;

impl Solution {
    pub fn max_length(nums: Vec<i32>) -> i32 {
        // The LCM of any subarray divides the LCM of the whole array and its GCD
        // is at most the largest element, so `lcm * gcd` never exceeds `limit`.
        let mut all_lcm = 1_i64;
        let mut max_num = 0_i64;
        for value in &nums {
            let x = i64::from(*value);
            max_num = max_num.max(x);
            all_lcm = all_lcm
                .checked_mul(x / Self::gcd(all_lcm, x))
                .unwrap_or(i64::MAX);
        }
        let limit = all_lcm.saturating_mul(max_num);

        let mut best = 0;

        for start in 0..nums.len() {
            let (mut prod, mut gcd, mut lcm) = (1_i64, 0_i64, 1_i64);

            for (offset, value) in nums[start..].iter().enumerate() {
                let x = i64::from(*value);

                // `x >= 1`, so the product only grows: once it is out of reach,
                // every longer subarray starting here fails too.
                if prod > limit / x.max(1) {
                    break;
                }
                prod *= x;
                gcd = Self::gcd(gcd, x);
                lcm = lcm / Self::gcd(lcm, x) * x;

                // `lcm` always divides `prod`, so `prod == lcm * gcd` holds exactly
                // when their quotient equals `gcd`.
                if prod / lcm == gcd {
                    best = best.max(offset as i32 + 1);
                }
            }
        }

        best
    }

    fn gcd(mut a: i64, mut b: i64) -> i64 {
        while b != 0 {
            let rest = a % b;
            a = b;
            b = rest;
        }

        a
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_subarray_with_equal_products::Solution;

    #[test]
    fn test_max_length_1() {
        let nums: Vec<i32> = vec![1, 2, 1, 2, 1, 1, 1];
        assert_eq!(5, Solution::max_length(nums));
    }

    #[test]
    fn test_max_length_2() {
        let nums: Vec<i32> = vec![2, 3, 4, 5, 6];
        assert_eq!(3, Solution::max_length(nums));
    }

    #[test]
    fn test_max_length_3() {
        let nums: Vec<i32> = vec![1, 2, 3, 1, 4, 5, 1];
        assert_eq!(5, Solution::max_length(nums));
    }
}
