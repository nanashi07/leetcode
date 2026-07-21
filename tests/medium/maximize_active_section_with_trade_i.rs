// 3499. Maximize Active Section with Trade I
// https://leetcode.com/problems/maximize-active-section-with-trade-i/

struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let mut ans = 0;
        let mut i = 0;
        let mut pre: Option<i32> = None;
        let mut mx = 0;

        while i < n {
            let mut j = i + 1;
            while j < n && s[j] == s[i] {
                j += 1;
            }
            let cur = (j - i) as i32;
            if s[i] == b'1' {
                ans += cur;
            } else {
                if let Some(p) = pre {
                    mx = mx.max(p + cur);
                }
                pre = Some(cur);
            }
            i = j;
        }

        ans + mx
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximize_active_section_with_trade_i::Solution;

    #[test]
    fn test_max_active_sections_after_trade_1() {
        let s = "01".to_string();
        assert_eq!(1, Solution::max_active_sections_after_trade(s));
    }

    #[test]
    fn test_max_active_sections_after_trade_2() {
        let s = "1000100".to_string();
        assert_eq!(7, Solution::max_active_sections_after_trade(s));
    }

    #[test]
    fn test_max_active_sections_after_trade_3() {
        let s = "01010".to_string();
        assert_eq!(4, Solution::max_active_sections_after_trade(s));
    }
}
