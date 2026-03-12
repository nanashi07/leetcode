// 3600. Maximize Spanning Tree Stability with Upgrades
// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades/

struct Solution;

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as i64;

        // Union-Find helpers (iterative path-halving)
        fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }

        fn union(parent: &mut Vec<usize>, rank: &mut Vec<u8>, x: usize, y: usize) -> bool {
            let px = find(parent, x);
            let py = find(parent, y);
            if px == py {
                return false;
            }
            if rank[px] < rank[py] {
                parent[px] = py;
            } else if rank[px] > rank[py] {
                parent[py] = px;
            } else {
                parent[py] = px;
                rank[px] += 1;
            }
            true
        }

        // Step 1: check if a spanning tree is reachable at all (ignoring weights).
        // Cost to use an edge = flag (0 for stable, 1 for unstable/needs activation).
        {
            let mut edge_list: Vec<(i64, usize, usize)> = edges
                .iter()
                .map(|e| (e[3] as i64, e[0] as usize, e[1] as usize))
                .collect();
            edge_list.sort_unstable();

            let mut parent: Vec<usize> = (0..n).collect();
            let mut rank = vec![0u8; n];
            let mut total_cost = 0i64;
            let mut used = 0usize;

            for (cost, u, v) in &edge_list {
                if union(&mut parent, &mut rank, *u, *v) {
                    total_cost += cost;
                    used += 1;
                    if used == n - 1 {
                        break;
                    }
                }
            }

            if used < n - 1 || total_cost > k {
                return -1;
            }
        }

        // Step 2: binary search on the stability value `mid`.
        // For a given `mid`, compute the minimum upgrade cost to build a spanning tree
        // where every edge has effective weight >= mid:
        //   flag=0, w >= mid        -> cost 0 (no upgrade needed)
        //   flag=0, 2w >= mid > w   -> cost 1 (boost: double weight)
        //   flag=1, w >= mid        -> cost 1 (activate)
        //   flag=1, 2w >= mid > w   -> cost 2 (activate + boost)
        //   otherwise               -> edge unusable for this mid
        let can_achieve = |mid: i64| -> bool {
            let mut edge_costs: Vec<(i64, usize, usize)> = Vec::new();

            for e in &edges {
                let (u, v, w, flag) = (e[0] as usize, e[1] as usize, e[2] as i64, e[3] as i64);
                let cost = if flag == 0 {
                    if w >= mid {
                        0
                    } else if 2 * w >= mid {
                        1
                    } else {
                        continue;
                    }
                } else {
                    if w >= mid {
                        1
                    } else if 2 * w >= mid {
                        2
                    } else {
                        continue;
                    }
                };
                edge_costs.push((cost, u, v));
            }

            // Minimum-cost spanning tree (Kruskal's by upgrade cost)
            edge_costs.sort_unstable();

            let mut parent: Vec<usize> = (0..n).collect();
            let mut rank = vec![0u8; n];
            let mut total_cost = 0i64;
            let mut used = 0usize;

            for (cost, u, v) in &edge_costs {
                if union(&mut parent, &mut rank, *u, *v) {
                    total_cost += cost;
                    if total_cost > k {
                        return false;
                    }
                    used += 1;
                    if used == n - 1 {
                        return true;
                    }
                }
            }

            false
        };

        let max_w = edges.iter().map(|e| e[2] as i64).max().unwrap_or(0);
        let mut lo = 1i64;
        let mut hi = 2 * max_w;
        let mut ans = 0i64;

        while lo <= hi {
            let mid = lo + (hi - lo) / 2;
            if can_achieve(mid) {
                ans = mid;
                lo = mid + 1;
            } else {
                hi = mid - 1;
            }
        }

        if ans == 0 {
            -1
        } else {
            ans as i32
        }
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
