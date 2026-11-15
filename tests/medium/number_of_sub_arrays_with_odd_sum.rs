// 1524. Number of Sub-arrays With Odd Sum
// https://leetcode.com/problems/number-of-sub-arrays-with-odd-sum/

struct Solution;

impl Solution {
    pub fn num_of_subarrays(arr: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let mut odd_prefixes: i64 = 0; // prefixes with odd sum parity
        let mut even_prefixes: i64 = 1; // pref[0] == 0 is even
        let mut parity: u8 = 0; // running prefix-sum parity (mod 2)

        for x in arr {
            parity ^= (x & 1) as u8;
            if parity == 0 {
                even_prefixes += 1;
            } else {
                odd_prefixes += 1;
            }
        }

        // A subarray sum is odd iff its two bounding prefixes have opposite parities.
        ((odd_prefixes * even_prefixes) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_sub_arrays_with_odd_sum::Solution;

    #[test]
    fn test_num_of_subarrays_1() {
        let arr: Vec<i32> = vec![1, 3, 5];
        assert_eq!(4, Solution::num_of_subarrays(arr));
    }

    #[test]
    fn test_num_of_subarrays_2() {
        let arr: Vec<i32> = vec![2, 4, 6];
        assert_eq!(0, Solution::num_of_subarrays(arr));
    }

    #[test]
    fn test_num_of_subarrays_3() {
        let arr: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
        assert_eq!(16, Solution::num_of_subarrays(arr));
    }
}
