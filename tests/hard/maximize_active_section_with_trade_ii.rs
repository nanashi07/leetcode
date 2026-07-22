// 3501. Maximize Active Section with Trade II
// https://leetcode.com/problems/maximize-active-section-with-trade-ii/

struct Solution;

impl Solution {
    pub fn max_active_sections_after_trade(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let bytes = s.as_bytes();
        let n = bytes.len();
        let total_ones = bytes.iter().filter(|&&b| b == b'1').count() as i32;

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

        let m = zero_sections.len();
        let pair_sums: Vec<i32> = if m >= 2 {
            (0..m - 1)
                .map(|i| {
                    let a = (zero_sections[i].1 - zero_sections[i].0 + 1) as i32;
                    let b = (zero_sections[i + 1].1 - zero_sections[i + 1].0 + 1) as i32;
                    a + b
                })
                .collect()
        } else {
            vec![]
        };

        let log_table = {
            let len = pair_sums.len();
            let mut log = vec![0u32; len + 1];
            for i in 2..=len {
                log[i] = log[i / 2] + 1;
            }
            log
        };

        let sparse = {
            let len = pair_sums.len();
            if len == 0 {
                vec![]
            } else {
                let max_log = log_table[len] as usize + 1;
                let mut sp = vec![vec![0i32; len]; max_log];
                sp[0] = pair_sums.clone();
                for k in 1..max_log {
                    for i in 0..len {
                        if i + (1 << k) <= len {
                            sp[k][i] = sp[k - 1][i].max(sp[k - 1][i + (1 << (k - 1))]);
                        }
                    }
                }
                sp
            }
        };

        let query_max = |l: usize, r: usize| -> i32 {
            if l > r || sparse.is_empty() {
                return 0;
            }
            let k = log_table[r - l + 1] as usize;
            sparse[k][l].max(sparse[k][r + 1 - (1 << k)])
        };

        queries
            .iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize;
                let lo = zero_sections.partition_point(|&(_, end)| end < l);
                let hi_exc = zero_sections.partition_point(|&(start, _)| start <= r);
                if hi_exc == 0 || lo >= hi_exc {
                    return total_ones;
                }
                let hi = hi_exc - 1;
                if hi <= lo {
                    return total_ones;
                }
                let eff_lo = (zero_sections[lo].1 - zero_sections[lo].0.max(l) + 1) as i32;
                let eff_hi = (zero_sections[hi].1.min(r) - zero_sections[hi].0 + 1) as i32;

                let mut best = 0i32;

                if hi == lo + 1 {
                    best = eff_lo + eff_hi;
                } else {
                    let full_lo1 = (zero_sections[lo + 1].1 - zero_sections[lo + 1].0 + 1) as i32;
                    let full_hi1 = (zero_sections[hi - 1].1 - zero_sections[hi - 1].0 + 1) as i32;
                    best = best.max(eff_lo + full_lo1);
                    best = best.max(full_hi1 + eff_hi);
                    if hi >= lo + 3 {
                        best = best.max(query_max(lo + 1, hi - 2));
                    }
                }

                total_ones + best
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

    #[test]
    fn test_max_active_sections_after_trade_5() {
        let s = "0101010101".to_string();
        let queries = to_vec2d([[0, 9], [0, 5], [0, 7]]);
        assert_eq!(
            [7, 7, 7].to_vec(),
            Solution::max_active_sections_after_trade(s, queries)
        );
    }
}
