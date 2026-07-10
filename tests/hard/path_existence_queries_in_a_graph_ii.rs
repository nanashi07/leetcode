// 3534. Path Existence Queries in a Graph II
// https://leetcode.com/problems/path-existence-queries-in-a-graph-ii/

struct Solution;

impl Solution {
    pub fn path_existence_queries(
        n: i32,
        nums: Vec<i32>,
        max_diff: i32,
        queries: Vec<Vec<i32>>,
    ) -> Vec<i32> {
        let n = n as usize;
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by_key(|&i| nums[i]);

        let mut pos = vec![0usize; n];
        for (i, &idx) in order.iter().enumerate() {
            pos[idx] = i;
        }

        let log = if n > 1 { (n as f64).log2() as usize + 1 } else { 1 };

        let mut reach = vec![vec![0usize; n]; log];

        let mut r = 0usize;
        for i in 0..n {
            if r < i {
                r = i;
            }
            while r + 1 < n && nums[order[r + 1]] - nums[order[i]] <= max_diff {
                r += 1;
            }
            reach[0][i] = r;
        }

        for k in 1..log {
            for i in 0..n {
                reach[k][i] = reach[k - 1][reach[k - 1][i]];
            }
        }

        queries
            .iter()
            .map(|q| {
                let mut a = pos[q[0] as usize];
                let mut b = pos[q[1] as usize];
                if a > b {
                    std::mem::swap(&mut a, &mut b);
                }
                if a == b {
                    return 0;
                }
                if reach[log - 1][a] < b {
                    return -1;
                }
                let mut steps = 0i32;
                let mut cur = a;
                for k in (0..log).rev() {
                    if reach[k][cur] < b {
                        cur = reach[k][cur];
                        steps += 1 << k;
                    }
                }
                steps + 1
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::path_existence_queries_in_a_graph_ii::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_path_existence_queries_1() {
        let n = 5;
        let nums = [1, 8, 3, 4, 2].to_vec();
        let max_diff = 3;
        let queries = to_vec2d([[0, 3], [2, 4]]);
        assert_eq!(
            [1, 1].to_vec(),
            Solution::path_existence_queries(n, nums, max_diff, queries)
        );
    }

    #[test]
    fn test_path_existence_queries_2() {
        let n = 5;
        let nums = [5, 3, 1, 9, 10].to_vec();
        let max_diff = 2;
        let queries = to_vec2d([[0, 1], [0, 2], [2, 3], [4, 3]]);
        assert_eq!(
            [1, 2, -1, 1].to_vec(),
            Solution::path_existence_queries(n, nums, max_diff, queries)
        );
    }

    #[test]
    fn test_path_existence_queries_3() {
        let n = 3;
        let nums = [3, 6, 1].to_vec();
        let max_diff = 1;
        let queries = to_vec2d([[0, 0], [0, 1], [1, 2]]);
        assert_eq!(
            [0, -1, -1].to_vec(),
            Solution::path_existence_queries(n, nums, max_diff, queries)
        );
    }
}
