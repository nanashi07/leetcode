// 2916. Subarrays Distinct Element Sum of Squares II
// https://leetcode.com/problems/subarrays-distinct-element-sum-of-squares-ii/

use std::collections::HashMap;

struct Solution;

const MOD: i64 = 1_000_000_007;

/// Lazy segment tree over every possible left border `l` of a subarray that ends
/// at the current right border. Leaf `l` holds `c_l = distinct(nums[l..=r])`;
/// each node stores `sum c_l`, `sum c_l^2` and a pending range increment, so a
/// `+v` on a range updates the squares with `(c+v)^2 = c^2 + 2vc + v^2`.
struct DistinctTree {
    sum: Vec<i64>,
    sq: Vec<i64>,
    lazy: Vec<i64>,
}

impl DistinctTree {
    fn apply(&mut self, o: usize, len: i64, v: i64) {
        self.sq[o] = (self.sq[o] + 2 * v % MOD * self.sum[o] + v * v % MOD * len) % MOD;
        self.sum[o] = (self.sum[o] + v * len) % MOD;
        self.lazy[o] = (self.lazy[o] + v) % MOD;
    }

    fn push_down(&mut self, o: usize, l: usize, r: usize) {
        let v = self.lazy[o];
        if v != 0 {
            let m = (l + r) / 2;
            self.apply(o * 2, (m - l + 1) as i64, v);
            self.apply(o * 2 + 1, (r - m) as i64, v);
            self.lazy[o] = 0;
        }
    }

    fn add(&mut self, o: usize, l: usize, r: usize, ql: usize, qr: usize) {
        if ql <= l && r <= qr {
            self.apply(o, (r - l + 1) as i64, 1);
            return;
        }
        self.push_down(o, l, r);
        let m = (l + r) / 2;
        if ql <= m {
            self.add(o * 2, l, m, ql, qr);
        }
        if qr > m {
            self.add(o * 2 + 1, m + 1, r, ql, qr);
        }
        self.sum[o] = (self.sum[o * 2] + self.sum[o * 2 + 1]) % MOD;
        self.sq[o] = (self.sq[o * 2] + self.sq[o * 2 + 1]) % MOD;
    }
}

impl Solution {
    /// Sweep the right border once, adding 1 to every left border that makes the
    /// new value a fresh distinct element. O(n log n) time, O(n) space.
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        let mut tree = DistinctTree {
            sum: vec![0; 4 * n + 5],
            sq: vec![0; 4 * n + 5],
            lazy: vec![0; 4 * n + 5],
        };
        let mut last: HashMap<i32, usize> = HashMap::with_capacity(n);
        let mut ans: i64 = 0;

        for (r, &x) in nums.iter().enumerate() {
            // `x` is new exactly for left borders after its previous occurrence.
            let ql = last.insert(x, r).map_or(0, |p| p + 1);
            tree.add(1, 0, n - 1, ql, r);
            ans += tree.sq[1];
            if ans >= MOD {
                ans -= MOD;
            }
        }

        ans as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::subarrays_distinct_element_sum_of_squares_ii::Solution;

    #[test]
    fn test_sum_counts_1() {
        let nums = [1, 2, 1].to_vec();
        assert_eq!(15, Solution::sum_counts(nums));
    }

    #[test]
    fn test_sum_counts_2() {
        let nums = [2, 2].to_vec();
        assert_eq!(3, Solution::sum_counts(nums));
    }
}
