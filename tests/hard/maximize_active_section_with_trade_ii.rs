// 3501. Maximize Active Section with Trade II
// https://leetcode.com/problems/maximize-active-section-with-trade-ii/

struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let total_ones = bytes.iter().filter(|&&b| b == b'1').count() as i32;

        let mut prefix_zeros = vec![0i32; n + 1];
        for i in 0..n {
            prefix_zeros[i + 1] = prefix_zeros[i] + if bytes[i] == b'0' { 1 } else { 0 };
        }

        let mut zero_sections: Vec<(usize, usize)> = Vec::new();
        let mut i = 0;
        while i < n {
            if bytes[i] == b'0' {
                let start = i;
                while i < n && bytes[i] == b'0' {
                    i += 1;
                }
                zero_sections.push((start, i - 1));
            } else {
                i += 1;
            }
        }

        queries
            .iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize;
                let lo = zero_sections.partition_point(|&(_, end)| end < l);
                let hi = zero_sections.partition_point(|&(start, _)| start <= r);
                if hi >= lo + 2 {
                    total_ones + prefix_zeros[r + 1] - prefix_zeros[l]
                } else {
                    total_ones
                }
            })
            .collect()
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
