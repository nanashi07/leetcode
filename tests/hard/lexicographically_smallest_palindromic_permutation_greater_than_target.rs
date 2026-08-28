// 3734. Lexicographically Smallest Palindromic Permutation Greater Than Target
// https://leetcode.com/problems/lexicographically-smallest-palindromic-permutation-greater-than-target/

struct Solution;

impl Solution {
    pub fn lex_palindromic_permutation(s: String, target: String) -> String {
        let mut counts = [0; 26];
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }

        let mut odd_count = 0;
        let mut mid_char = None;
        for (i, &count) in counts.iter().enumerate() {
            if count % 2 != 0 {
                odd_count += 1;
                mid_char = Some((i as u8 + b'a') as char);
            }
        }

        if odd_count > 1 {
            return "".to_string();
        }

        let mut current_avail = [0; 26];
        for (i, &count) in counts.iter().enumerate() {
            current_avail[i] = count / 2;
        }

        let half_n = s.len() / 2;
        let mut matched_all = true;
        let mut best_diverge: Option<(usize, u8)> = None;

        for i in 0..half_n {
            let tc = if i < target.len() {
                target.as_bytes()[i]
            } else {
                0
            };

            let start = if tc < b'a' { b'a' } else { tc + 1 };
            for c in start..=b'z' {
                if current_avail[(c - b'a') as usize] > 0 {
                    best_diverge = Some((i, c));
                    break;
                }
            }

            if tc.is_ascii_lowercase() && current_avail[(tc - b'a') as usize] > 0 {
                current_avail[(tc - b'a') as usize] -= 1;
            } else {
                matched_all = false;
                break;
            }
        }

        if matched_all {
            let mut p_match = String::with_capacity(s.len());
            let prefix = &target[..half_n];
            p_match.push_str(prefix);
            if let Some(m) = mid_char {
                p_match.push(m);
            }
            let rev: String = prefix.chars().rev().collect();
            p_match.push_str(&rev);

            if p_match > target {
                return p_match;
            }
        }

        if let Some((idx, c)) = best_diverge {
            let mut l = String::with_capacity(half_n);
            let prefix = &target[..idx];
            l.push_str(prefix);
            l.push(c as char);

            let mut rem_avail = [0; 26];
            for (i, &count) in counts.iter().enumerate() {
                rem_avail[i] = count / 2;
            }
            for b in prefix.bytes() {
                rem_avail[(b - b'a') as usize] -= 1;
            }
            rem_avail[(c - b'a') as usize] -= 1;

            for (i, &avail) in rem_avail.iter().enumerate() {
                for _ in 0..avail {
                    l.push((i as u8 + b'a') as char);
                }
            }

            let mut p = String::with_capacity(s.len());
            p.push_str(&l);
            if let Some(m) = mid_char {
                p.push(m);
            }
            let rev: String = l.chars().rev().collect();
            p.push_str(&rev);
            return p;
        }

        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::lexicographically_smallest_palindromic_permutation_greater_than_target::Solution;

    #[test]
    fn test_lex_palindromic_permutation_1() {
        let s = "baba".to_string();
        let target = "abba".to_string();
        assert_eq!(
            "baab".to_string(),
            Solution::lex_palindromic_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_palindromic_permutation_2() {
        let s = "baba".to_string();
        let target = "bbaa".to_string();
        assert_eq!(
            "".to_string(),
            Solution::lex_palindromic_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_palindromic_permutation_3() {
        let s = "abc".to_string();
        let target = "abb".to_string();
        assert_eq!(
            "".to_string(),
            Solution::lex_palindromic_permutation(s, target)
        );
    }

    #[test]
    fn test_lex_palindromic_permutation_4() {
        let s = "aac".to_string();
        let target = "abb".to_string();
        assert_eq!(
            "aca".to_string(),
            Solution::lex_palindromic_permutation(s, target)
        );
    }
}
