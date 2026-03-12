// 3600. Maximize Spanning Tree Stability with Upgrades
// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades/

struct Solution;

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_spanning_tree_stability_with_upgrades::Solution;

    #[test]
    fn test_max_stability_1() {
        let n = 3;
        let edges = [[0, 1, 2, 1], [1, 2, 3, 0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 1;
        assert_eq!(2, Solution::max_stability(n, edges, k));
    }

    #[test]
    fn test_max_stability_2() {
        let n = 3;
        let edges = [[0, 1, 4, 0], [1, 2, 3, 0], [0, 2, 1, 0]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 2;
        assert_eq!(6, Solution::max_stability(n, edges, k));
    }

    #[test]
    fn test_max_stability_3() {
        let n = 3;
        let edges = [[0, 1, 1, 1], [1, 2, 1, 1], [2, 0, 1, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let k = 0;
        assert_eq!(-1, Solution::max_stability(n, edges, k));
    }
}
