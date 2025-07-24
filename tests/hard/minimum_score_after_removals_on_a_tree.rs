// # 2322. Minimum Score After Removals on a Tree
// https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/

struct Solution;

use std::cmp::{max, min};

impl Solution {
    fn calc(a: i32, b: i32, c: i32) -> i32 {
        max(a, max(b, c)) - min(a, min(b, c))
    }

    // https://leetcode.com/problems/minimum-score-after-removals-on-a-tree/editorial/
    pub fn minimum_score(nums: Vec<i32>, edges: Vec<Vec<i32>>) -> i32 {
        let n = nums.len();
        let mut e = vec![vec![]; n];
        for v in edges.iter() {
            e[v[0] as usize].push(v[1] as usize);
            e[v[1] as usize].push(v[0] as usize);
        }

        let mut total = 0;
        for &x in nums.iter() {
            total ^= x;
        }

        let mut res = i32::MAX;

        fn dfs2(
            x: usize,
            f: usize,
            oth: i32,
            anc: usize,
            nums: &Vec<i32>,
            e: &Vec<Vec<usize>>,
            res: &mut i32,
            total: i32,
        ) -> i32 {
            let mut son = nums[x];
            for &y in &e[x] {
                if y == f {
                    continue;
                }
                son ^= dfs2(y, x, oth, anc, nums, e, res, total);
            }
            if f == anc {
                return son;
            }
            *res = min(*res, Solution::calc(oth, son, total ^ oth ^ son));
            son
        }

        fn dfs(
            x: usize,
            f: usize,
            nums: &Vec<i32>,
            e: &Vec<Vec<usize>>,
            res: &mut i32,
            total: i32,
        ) -> i32 {
            let mut son = nums[x];
            for &y in &e[x] {
                if y == f {
                    continue;
                }
                son ^= dfs(y, x, nums, e, res, total);
            }
            for &y in &e[x] {
                if y == f {
                    dfs2(y, x, son, x, nums, e, res, total);
                }
            }
            son
        }

        dfs(0, usize::MAX, &nums, &e, &mut res, total);
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_score_after_removals_on_a_tree::Solution;

    #[test]
    fn test_minimum_score_1() {
        let nums = [1, 5, 5, 4, 11].to_vec();
        let edges = [[0, 1], [1, 2], [1, 3], [3, 4]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(9, Solution::minimum_score(nums, edges));
    }

    #[test]
    fn test_minimum_score_2() {
        let nums = [5, 5, 2, 4, 4, 2].to_vec();
        let edges = [[0, 1], [1, 2], [5, 2], [4, 3], [1, 3]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(0, Solution::minimum_score(nums, edges));
    }
}
