// 3499. Maximize Active Section with Trade I
// https://leetcode.com/problems/maximize-active-section-with-trade-i/

struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String) -> i32 {
        todo!()
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
