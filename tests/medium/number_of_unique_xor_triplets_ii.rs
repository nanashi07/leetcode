// 3514. Number of Unique XOR Triplets II
// https://leetcode.com/problems/number-of-unique-xor-triplets-ii/

struct Solution;

impl Solution {
    pub fn unique_xor_triplets(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let max_val = *nums.iter().max().unwrap_or(&0);
        if max_val == 0 {
            return 1; // [0] -> 0 ^ 0 ^ 0 = 0
        }
        // If max value is max_val, the XOR of any two elements can be at most next power of 2 minus 1.
        // Let's find the smallest power of 2 greater than max_val.
        // Since max_val <= 1500, the next power of 2 is at most 2048.
        let mut limit = 1;
        while limit <= max_val {
            limit <<= 1;
        }
        // Since we are XORing up to 3 elements, the maximum possible XOR value is less than 2 * limit.
        // But actually, each element is <= max_val < limit.
        // So any element is < limit.
        // The XOR of two elements is < limit.
        // The XOR of three elements is also < limit (since a ^ b ^ c < limit as well, as each bit position
        // is at most the maximum bit position of the numbers, i.e., < limit).
        // Let's verify: if a, b, c < limit, then their XOR is also < limit.
        // So the maximum XOR value is at most limit - 1.
        // Thus, the size of our tables can be limit.

        let limit = limit as usize;
        let mut st = vec![false; limit];
        for &a in &nums {
            for &b in &nums {
                st[(a ^ b) as usize] = true;
            }
        }

        let mut s = vec![false; limit];
        for ab in 0..limit {
            if st[ab] {
                for &c in &nums {
                    s[ab ^ (c as usize)] = true;
                }
            }
        }

        s.iter().filter(|&&v| v).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_unique_xor_triplets_ii::Solution;

    #[test]
    fn test_unique_xor_triplets_1() {
        let nums = [1, 3].to_vec();
        assert_eq!(2, Solution::unique_xor_triplets(nums));
    }

    #[test]
    fn test_unique_xor_triplets_2() {
        let nums = [6, 7, 8, 9].to_vec();
        assert_eq!(4, Solution::unique_xor_triplets(nums));
    }
}
