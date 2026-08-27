// 3720. Lexicographically Smallest Permutation Greater Than Target
// https://leetcode.com/problems/lexicographically-smallest-permutation-greater-than-target/

struct Solution;

impl Solution {
    pub fn lex_greater_permutation(s: String, target: String) -> String {
        // ponytail: assume ASCII input, upgrade path is using char counts/indices if UTF-8 support is needed.
        let s_bytes = s.as_bytes();
        let t_bytes = target.as_bytes();
        let n = s_bytes.len();

        let mut freq = [0u32; 256];
        for &b in s_bytes {
            freq[b as usize] += 1;
        }

        let mut l = 0;
        while l < n {
            let b = t_bytes[l] as usize;
            if freq[b] > 0 {
                freq[b] -= 1;
                l += 1;
            } else {
                break;
            }
        }

        if n == 0 {
            return String::new();
        }

        let mut len = l.min(n - 1);
        loop {
            if len < l {
                freq[t_bytes[len] as usize] += 1;
            }

            let target_char = t_bytes[len] as usize;
            let mut found_char = None;
            for (d, &count) in freq.iter().enumerate().skip(target_char + 1) {
                if count > 0 {
                    found_char = Some(d);
                    break;
                }
            }

            if let Some(d) = found_char {
                let mut res = Vec::with_capacity(n);
                res.extend_from_slice(&t_bytes[0..len]);
                res.push(d as u8);
                freq[d] -= 1;
                for (c, &count) in freq.iter().enumerate() {
                    for _ in 0..count {
                        res.push(c as u8);
                    }
                }
                return String::from_utf8(res).unwrap();
            }

            if len == 0 {
                break;
            }
            len -= 1;
        }

        String::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::lexicographically_smallest_permutation_greater_than_target::Solution;

    #[test]
    fn test_lex_greater_permutation_1() {
        let s = "abc".to_string();
        let target = "bba".to_string();
        assert_eq!(
            "bca".to_string(),
            Solution::lex_greater_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_greater_permutation_2() {
        let s = "leet".to_string();
        let target = "code".to_string();
        assert_eq!(
            "eelt".to_string(),
            Solution::lex_greater_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_greater_permutation_3() {
        let s = "baba".to_string();
        let target = "bbaa".to_string();
        assert_eq!("".to_string(), Solution::lex_greater_permutation(s, target));
    }
}
