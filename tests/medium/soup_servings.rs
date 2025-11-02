// 808. Soup Servings
// https://leetcode.com/problems/soup-servings/

struct Solution;

use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    // https://leetcode.com/problems/soup-servings/editorial/
    pub fn soup_servings(n: i32) -> f64 {
        let m = ((n as f64) / 25.0).ceil() as i32;
        let mut dp: HashMap<i32, HashMap<i32, f64>> = HashMap::new();

        // Initialize dp[0][0]
        dp.entry(0).or_insert_with(HashMap::new).insert(0, 0.5);

        // Helper function to calculate DP value
        let calculate_dp = |dp: &HashMap<i32, HashMap<i32, f64>>, i: i32, j: i32| -> f64 {
            let get_dp_value = |dp: &HashMap<i32, HashMap<i32, f64>>, x: i32, y: i32| -> f64 {
                dp.get(&x)
                    .and_then(|row| row.get(&y))
                    .copied()
                    .unwrap_or(0.0)
            };

            (get_dp_value(dp, max(0, i - 4), j)
                + get_dp_value(dp, max(0, i - 3), j - 1)
                + get_dp_value(dp, max(0, i - 2), max(0, j - 2))
                + get_dp_value(dp, i - 1, max(0, j - 3)))
                / 4.0
        };

        for k in 1..=m {
            // dp[0][k] = 1
            dp.entry(0).or_insert_with(HashMap::new).insert(k, 1.0);
            // dp[k][0] = 0
            dp.entry(k).or_insert_with(HashMap::new).insert(0, 0.0);

            for j in 1..=k {
                let val_jk = calculate_dp(&dp, j, k);
                dp.entry(j).or_insert_with(HashMap::new).insert(k, val_jk);

                let val_kj = calculate_dp(&dp, k, j);
                dp.entry(k).or_insert_with(HashMap::new).insert(j, val_kj);
            }

            let dp_kk = dp
                .get(&k)
                .and_then(|row| row.get(&k))
                .copied()
                .unwrap_or(0.0);
            if dp_kk > 1.0 - 1e-5 {
                return 1.0;
            }
        }

        dp.get(&m)
            .and_then(|row| row.get(&m))
            .copied()
            .unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::soup_servings::Solution;

    #[test]
    fn test_soup_servings_1() {
        let n = 50;
        assert_eq!(0.62500, Solution::soup_servings(n));
    }

    #[test]
    fn test_soup_servings_2() {
        let n = 100;
        assert_eq!(0.71875, Solution::soup_servings(n));
    }
}
