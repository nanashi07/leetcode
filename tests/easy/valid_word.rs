// 3136. Valid Word
// https://leetcode.com/problems/valid-word/

struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        if word.len() < 3 {
            return false;
        }

        let mut vc = 0;
        let mut cc = 0;

        // 0-9: 48 - 57
        // A-Z: 65 - 90
        // a-z: 97 - 122
        // vowel: 65, 69, 73, 79, 85
        let vowel = [65, 69, 73, 79, 85];

        for c in word.chars() {
            let n = c as u32;
            if (48..=57).contains(&n) || (65..=90).contains(&n) || (97..=122).contains(&n) {
                if n >= 65 {
                    if vowel.contains(&n) || vowel.contains(&(n + 65 - 97)) {
                        vc += 1;
                    } else {
                        cc += 1;
                    }
                }
            } else {
                return false;
            }
        }

        vc > 0 && cc > 0
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::valid_word::Solution;

    #[test]
    fn test_is_valid_1() {
        let word = "234Adas".to_owned();
        assert!(Solution::is_valid(word));
    }

    #[test]
    fn test_is_valid_2() {
        let word = "b3".to_owned();
        assert!(!Solution::is_valid(word));
    }

    #[test]
    fn test_is_valid_3() {
        let word = "a3$e".to_owned();
        assert!(!Solution::is_valid(word));
    }
}
