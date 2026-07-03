// 3620. Network Recovery Pathways
// https://leetcode.com/problems/network-recovery-pathways/

struct Solution;

impl Solution {
    pub fn find_max_path_score(edges: Vec<Vec<i32>>, online: Vec<bool>, k: i64) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = online.len();
        let target = n - 1;
        let mut adj = vec![vec![]; n];
        for e in &edges {
            let (u, v, w) = (e[0] as usize, e[1] as usize, e[2] as i64);
            if online[u] && online[v] {
                adj[u].push((v, w));
                adj[v].push((u, w));
            }
        }

        let mut lo = 0i32;
        let mut hi = 0i32;
        for e in &edges {
            hi = hi.max(e[2]);
        }
        let mut ans = -1;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            let threshold = mid as i64;
            let mut dist = vec![i64::MAX; n];
            dist[0] = 0;
            let mut heap = BinaryHeap::new();
            heap.push(Reverse((0i64, 0usize)));
            while let Some(Reverse((d, u))) = heap.pop() {
                if d > dist[u] {
                    continue;
                }
                for &(v, w) in &adj[u] {
                    if w >= threshold && d + w < dist[v] {
                        dist[v] = d + w;
                        heap.push(Reverse((dist[v], v)));
                    }
                }
            }
            if dist[target] <= k {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::network_recovery_pathways::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_find_max_path_score_1() {
        let edges = to_vec2d([[0, 1, 5], [1, 3, 10], [0, 2, 3], [2, 3, 4]]);
        let online = [true, true, true, true].to_vec();
        let k = 10;
        assert_eq!(3, Solution::find_max_path_score(edges, online, k));
    }

    #[test]
    fn test_find_max_path_score_2() {
        let edges = to_vec2d([
            [0, 1, 7],
            [1, 4, 5],
            [0, 2, 6],
            [2, 3, 6],
            [3, 4, 2],
            [2, 4, 6],
        ]);
        let online = [true, true, true, false, true].to_vec();
        let k = 12;
        assert_eq!(6, Solution::find_max_path_score(edges, online, k));
    }
}
