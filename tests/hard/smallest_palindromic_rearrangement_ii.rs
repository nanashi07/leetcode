// 3518. Smallest Palindromic Rearrangement II
// https://leetcode.com/problems/smallest-palindromic-rearrangement-ii/

struct Solution;

const CAP: u64 = 3_000_000_000;

impl Solution {
    pub fn smallest_palindrome(s: String, k: i32) -> String {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let half_len = n / 2;

        let mut freq = [0u64; 26];
        for &b in bytes {
            freq[(b - b'a') as usize] += 1;
        }

        let mut half_freq = [0u64; 26];
        let mut mid_char: Option<u8> = None;
        for i in 0..26 {
            half_freq[i] = freq[i] / 2;
            if freq[i] % 2 == 1 {
                mid_char = Some(i as u8);
            }
        }

        let total_perms = Self::multinomial_capped(half_len as u64, &half_freq);
        let k_u64 = k as u64;
        if k_u64 > total_perms {
            return String::new();
        }

        let half = Self::unrank_permutation(half_len, &mut half_freq, k_u64);

        let mut result = half.clone();
        if let Some(mc) = mid_char {
            result.push(b'a' + mc);
        }
        for &b in half.iter().rev() {
            result.push(b);
        }

        String::from_utf8(result).unwrap()
    }

    fn multinomial_capped(n: u64, freq: &[u64; 26]) -> u64 {
        let mut result: u128 = 1;
        let mut top = n as u128;
        for &f in freq {
            let f128 = f as u128;
            let k = f128.min(top - f128);
            let mut binom: u128 = 1;
            for j in 1..=k {
                binom = binom * (top - j + 1) / j;
                if binom > CAP as u128 {
                    return CAP;
                }
            }
            top -= f128;
            result *= binom;
            if result > CAP as u128 {
                return CAP;
            }
        }
        result as u64
    }

    fn unrank_permutation(len: usize, freq: &mut [u64; 26], mut rank: u64) -> Vec<u8> {
        let mut result = Vec::with_capacity(len);
        let mut remaining = len as u64;

        for _ in 0..len {
            for c in 0..26u8 {
                if freq[c as usize] == 0 {
                    continue;
                }
                freq[c as usize] -= 1;
                remaining -= 1;
                let count = Self::multinomial_capped(remaining, freq);
                if rank <= count {
                    result.push(b'a' + c);
                    break;
                }
                rank -= count;
                freq[c as usize] += 1;
                remaining += 1;
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::smallest_palindromic_rearrangement_ii::Solution;

    #[test]
    fn test_smallest_palindrome_1() {
        let s = "abba".to_string();
        let k = 2;
        assert_eq!("baab".to_string(), Solution::smallest_palindrome(s, k));
    }

    #[test]
    fn test_smallest_palindrome_2() {
        let s = "aa".to_string();
        let k = 2;
        assert_eq!("".to_string(), Solution::smallest_palindrome(s, k));
    }

    #[test]
    fn test_smallest_palindrome_3() {
        let s = "bacab".to_string();
        let k = 1;
        assert_eq!("abcba".to_string(), Solution::smallest_palindrome(s, k));
    }
}
