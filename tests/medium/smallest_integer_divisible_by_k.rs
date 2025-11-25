// 1015. Smallest Integer Divisible by K
// https://leetcode.com/problems/smallest-integer-divisible-by-k/

struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        // If k is divisible by 2 or 5, no repunit can be divisible by k
        // because repunits are of the form (10^n - 1)/9, and 10^n - 1 ends in 9
        if k % 2 == 0 || k % 5 == 0 {
            return -1;
        }

        let mut remainder = 0;

        // Try building repunits: 1, 11, 111, 1111, ...
        // Instead of building the actual number, we track remainder % k
        // For each new digit: new_number = old_number * 10 + 1
        // So: new_remainder = (old_remainder * 10 + 1) % k
        for length in 1..=k {
            remainder = (remainder * 10 + 1) % k;

            if remainder == 0 {
                return length;
            }
        }

        // Should never reach here for valid inputs
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_integer_divisible_by_k::Solution;

    #[test]
    fn test_smallest_repunit_div_by_k_1() {
        let k = 1;
        assert_eq!(1, Solution::smallest_repunit_div_by_k(k));
    }

    #[test]
    fn test_smallest_repunit_div_by_k_2() {
        let k = 2;
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(k));
    }

    #[test]
    fn test_smallest_repunit_div_by_k_3() {
        let k = 3;
        assert_eq!(3, Solution::smallest_repunit_div_by_k(k));
    }
}
