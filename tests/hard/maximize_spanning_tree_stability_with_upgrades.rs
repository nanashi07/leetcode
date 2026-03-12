// 3600. Maximize Spanning Tree Stability with Upgrades
// https://leetcode.com/problems/maximize-spanning-tree-stability-with-upgrades/

struct Solution;

impl Solution {
    pub fn max_stability(n: i32, edges: Vec<Vec<i32>>, k: i32) -> i32 {
        let n = n as usize;
        let k = k as i32;

        fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }

        fn union(parent: &mut Vec<usize>, size: &mut Vec<usize>, x: usize, y: usize) -> bool {
            let px = find(parent, x);
            let py = find(parent, y);
            if px == py {
                return false;
            }
            if size[px] >= size[py] {
                parent[py] = px;
                size[px] += size[py];
            } else {
                parent[px] = py;
                size[py] += size[px];
            }
            true
        }

        // Step 1: Add all must=1 edges. Detect cycles (cycle → invalid spanning tree → -1).
        // Compute mn = minimum strength among must=1 edges.
        let mut parent: Vec<usize> = (0..n).collect();
        let mut size = vec![1usize; n];
        let mut mn = i64::MAX;
        for e in &edges {
            if e[3] == 1 {
                let (u, v, s) = (e[0] as usize, e[1] as usize, e[2] as i64);
                mn = mn.min(s);
                if !union(&mut parent, &mut size, u, v) {
                    return -1; // must=1 edges form a cycle
                }
            }
        }

        // Step 2: Check that all nodes can be connected using all edges.
        let mut parent2: Vec<usize> = (0..n).collect();
        let mut size2 = vec![1usize; n];
        for e in &edges {
            union(&mut parent2, &mut size2, e[0] as usize, e[1] as usize);
        }
        let root = find(&mut parent2, 0);
        if (1..n).any(|i| find(&mut parent2, i) != root) {
            return -1; // graph is disconnected
        }

        // If no must=1 edges exist, the upper bound is 2 * max edge strength.
        if mn == i64::MAX {
            mn = 2 * edges.iter().map(|e| e[2] as i64).max().unwrap_or(0);
        }

        // check(lim): can we form a spanning tree with all edge strengths >= lim?
        //   - Free tier: edges with s >= lim (both must and optional)
        //   - Upgrade tier: edges with 2*s >= lim (optional only), costs 1 upgrade each, up to k
        let check = |lim: i64| -> bool {
            let mut parent: Vec<usize> = (0..n).collect();
            let mut size = vec![1usize; n];
            let mut cnt = n;

            for e in &edges {
                let (u, v, s) = (e[0] as usize, e[1] as usize, e[2] as i64);
                if s >= lim && union(&mut parent, &mut size, u, v) {
                    cnt -= 1;
                }
            }
            if cnt == 1 {
                return true;
            }
            let mut rem = k;
            for e in &edges {
                if rem == 0 {
                    break;
                }
                let (u, v, s) = (e[0] as usize, e[1] as usize, e[2] as i64);
                if 2 * s >= lim && union(&mut parent, &mut size, u, v) {
                    cnt -= 1;
                    rem -= 1;
                    if cnt == 1 {
                        return true;
                    }
                }
            }
            cnt == 1
        };

        // Binary search for the maximum feasible stability in [1, mn].
        let mut lo = 1i64;
        let mut hi = mn;
        while lo < hi {
            let mid = lo + (hi - lo + 1) / 2;
            if check(mid) {
                lo = mid;
            } else {
                hi = mid - 1;
            }
        }

        if check(lo) {
            lo as i32
        } else {
            -1
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

    #[test]
    fn test_max_stability_5() {
        let n = 5;
        let edges = [
            [3, 2, 56447, 1],
            [4, 3, 80497, 1],
            [0, 4, 45565, 1],
            [1, 2, 85317, 1],
            [0, 1, 87891, 0],
            [0, 2, 78889, 0],
            [2, 4, 73816, 0],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        let k = 4;
        assert_eq!(45565, Solution::max_stability(n, edges, k));
    }
}
