// 3501. Maximize Active Section with Trade II
// https://leetcode.com/problems/maximize-active-section-with-trade-ii/

struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_active_section_with_trade_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_active_sections_after_trade_1() {
        let s = "01".to_string();
        let queries = to_vec2d([[0, 1]]);
        assert_eq!(
            [1].to_vec(),
            Solution::max_active_sections_after_trade(s, queries)
        );
    }

    #[test]
    fn test_max_active_sections_after_trade_2() {
        let s = "0100".to_string();
        let queries = to_vec2d([[0, 3], [0, 2], [1, 3], [2, 3]]);
        assert_eq!(
            [4, 3, 1, 1].to_vec(),
            Solution::max_active_sections_after_trade(s, queries)
        );
    }

    #[test]
    fn test_max_active_sections_after_trade_3() {
        let s = "1000100".to_string();
        let queries = to_vec2d([[1, 5], [0, 6], [0, 4]]);
        assert_eq!(
            [6, 7, 2].to_vec(),
            Solution::max_active_sections_after_trade(s, queries)
        );
    }

    #[test]
    fn test_max_active_sections_after_trade_4() {
        let s = "01010".to_string();
        let queries = to_vec2d([[0, 3], [1, 4], [1, 3]]);
        assert_eq!(
            [4, 4, 2].to_vec(),
            Solution::max_active_sections_after_trade(s, queries)
        );
    }
}
