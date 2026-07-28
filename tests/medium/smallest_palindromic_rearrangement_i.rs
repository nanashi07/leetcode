// 3517. Smallest Palindromic Rearrangement I
// https://leetcode.com/problems/smallest-palindromic-rearrangement-i/

struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let mut cnt = [0; 26];
        for b in s.bytes() {
            cnt[(b - b'a') as usize] += 1;
        }

        let mut t = Vec::with_capacity(s.len() / 2);
        let mut ch = None;

        for (i, &count) in cnt.iter().enumerate() {
            let letter = (b'a' + i as u8) as char;
            let half = count / 2;
            for _ in 0..half {
                t.push(letter);
            }
            if count % 2 == 1 {
                ch = Some(letter);
            }
        }

        let mut ans = String::with_capacity(s.len());
        for &c in &t {
            ans.push(c);
        }
        if let Some(c) = ch {
            ans.push(c);
        }
        for &c in t.iter().rev() {
            ans.push(c);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_palindromic_rearrangement_i::Solution;

    #[test]
    fn test_smallest_palindrome_1() {
        let s = "z".to_string();
        assert_eq!("z".to_string(), Solution::smallest_palindrome(s));
    }

    #[test]
    fn test_smallest_palindrome_2() {
        let s = "babab".to_string();
        assert_eq!("abbba".to_string(), Solution::smallest_palindrome(s));
    }

    #[test]
    fn test_smallest_palindrome_3() {
        let s = "daccad".to_string();
        assert_eq!("acddca".to_string(), Solution::smallest_palindrome(s));
    }
}
