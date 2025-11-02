// 2561. Rearranging Fruits
// https://leetcode.com/problems/rearranging-fruits/

use std::collections::HashMap;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/rearranging-fruits/editorial/
    pub fn min_cost(basket1: Vec<i32>, basket2: Vec<i32>) -> i64 {
        let mut freq = HashMap::new();
        let mut m = i32::MAX;
        for &b in &basket1 {
            *freq.entry(b).or_insert(0) += 1;
            m = m.min(b);
        }
        for &b in &basket2 {
            *freq.entry(b).or_insert(0) -= 1;
            m = m.min(b);
        }

        let mut merge = vec![];
        for (&k, &v) in freq.iter() {
            if v % 2 != 0 {
                return -1;
            }
            for _ in 0..((v as i32).abs() / 2) {
                merge.push(k);
            }
        }

        merge.sort_unstable();
        let res: i64 = merge
            .iter()
            .take(merge.len() / 2)
            .map(|&x| i64::from(x.min(2 * m)))
            .sum();
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::rearranging_fruits::Solution;

    #[test]
    fn test_min_cost_1() {
        let basket1 = [4, 2, 2, 2].to_vec();
        let basket2 = [1, 4, 1, 2].to_vec();
        assert_eq!(1, Solution::min_cost(basket1, basket2));
    }

    #[test]
    fn test_min_cost_2() {
        let basket1 = [2, 3, 4, 1].to_vec();
        let basket2 = [3, 2, 5, 1].to_vec();
        assert_eq!(-1, Solution::min_cost(basket1, basket2));
    }
}
