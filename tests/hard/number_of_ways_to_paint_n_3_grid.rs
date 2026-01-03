// 1411. Number of Ways to Paint N × 3 Grid
// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/

struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        // For the first row:
        // Pattern ABA (two colors): 6 ways (e.g., RGR, RBR, GRG, GBG, BRB, BGB)
        // Pattern ABC (three colors): 6 ways (e.g., RGB, RBG, GRB, GBR, BRG, BGR)
        let mut aba: i64 = 6; // Number of ways ending with ABA pattern
        let mut abc: i64 = 6; // Number of ways ending with ABC pattern

        // For each subsequent row, calculate valid transitions
        for _ in 1..n {
            // From ABA pattern: can transition to 3 ABA patterns and 2 ABC patterns
            // From ABC pattern: can transition to 2 ABA patterns and 2 ABC patterns
            let new_aba = (aba * 3 + abc * 2) % MOD;
            let new_abc = (aba * 2 + abc * 2) % MOD;

            aba = new_aba;
            abc = new_abc;
        }

        ((aba + abc) % MOD) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_ways_to_paint_n_3_grid::Solution;

    #[test]
    fn test_num_of_ways_1() {
        let n = 1;
        assert_eq!(12, Solution::num_of_ways(n));
    }

    #[test]
    fn test_num_of_ways_2() {
        let n = 5000;
        assert_eq!(30228214, Solution::num_of_ways(n));
    }
}
