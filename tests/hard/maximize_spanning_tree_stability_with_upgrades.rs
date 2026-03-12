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

        // Checks whether a spanning tree with every edge having effective weight >= mid
        // can be built within the given boost_budget.
        //
        // Two modes controlled by `include_flag1`:
        //   false (Case 1, k=0 scenario):
        //     Only flag=0 edges are available; they can be boosted (×2) at cost 1 each.
        //   true  (Case 2, k≥1 scenario):
        //     flag=1 edges are available at their original weight w (NOT boostable).
        //     flag=0 edges are available and can be boosted at cost 1.
        //     The "unlock" of all flag=1 edges has already been accounted for (budget = k-1).
        let check = |mid: i64, include_flag1: bool, boost_budget: i64| -> bool {
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
                    // flag == 1: usable only when include_flag1; cannot be boosted
                    if !include_flag1 {
                        continue;
                    }
                    if w >= mid {
                        0
                    } else {
                        continue; // w < mid and can't boost
                    }
                };
                edge_costs.push((cost, u, v));
            }

            // Minimum-cost spanning tree (Kruskal's, sorting by boost cost)
            edge_costs.sort_unstable();

            let mut parent: Vec<usize> = (0..n).collect();
            let mut rank = vec![0u8; n];
            let mut total_cost = 0i64;
            let mut used = 0usize;

            for (cost, u, v) in &edge_costs {
                if union(&mut parent, &mut rank, *u, *v) {
                    total_cost += cost;
                    if total_cost > boost_budget {
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
        let max_flag0_w = edges
            .iter()
            .filter(|e| e[3] == 0)
            .map(|e| e[2] as i64)
            .max()
            .unwrap_or(0);

        let mut ans = 0i64;

        // Case 1: only flag=0 edges, full boost budget k.
        // Maximum achievable stability = 2 * max_flag0_w.
        {
            let mut lo = 1i64;
            let mut hi = 2 * max_flag0_w;
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if check(mid, false, k) {
                    ans = ans.max(mid);
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
            }
        }

        // Case 2 (only when k ≥ 1): all edges available; 1 upgrade spent globally to unlock
        // all flag=1 edges, leaving k-1 upgrades for boosting flag=0 edges.
        // flag=1 edges cannot be boosted, so max achievable = max(max_w, 2*max_flag0_w).
        if k >= 1 {
            let mut lo = 1i64;
            let mut hi = max_w.max(2 * max_flag0_w);
            while lo <= hi {
                let mid = lo + (hi - lo) / 2;
                if check(mid, true, k - 1) {
                    ans = ans.max(mid);
                    lo = mid + 1;
                } else {
                    hi = mid - 1;
                }
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

    #[test]
    fn test_max_stability_4() {
        let n = 5;
        let edges = [
            [2, 0, 68643, 1],
            [1, 0, 31681, 1],
            [4, 2, 44760, 1],
            [3, 4, 19034, 1],
            [1, 4, 24247, 0],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let k = 2;
        assert_eq!(19034, Solution::max_stability(n, edges, k));
    }
}
