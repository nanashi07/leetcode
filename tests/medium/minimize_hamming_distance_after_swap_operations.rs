// 1722. Minimize Hamming Distance After Swap Operations
// https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/

struct Solution;

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        use std::collections::HashMap;

        let n = source.len();
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank = vec![0u32; n];

        fn find(parent: &mut Vec<usize>, x: usize) -> usize {
            if parent[x] != x {
                parent[x] = find(parent, parent[x]);
            }
            parent[x]
        }

        fn union(parent: &mut Vec<usize>, rank: &mut Vec<u32>, a: usize, b: usize) {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra == rb { return; }
            if rank[ra] < rank[rb] {
                parent[ra] = rb;
            } else if rank[ra] > rank[rb] {
                parent[rb] = ra;
            } else {
                parent[rb] = ra;
                rank[ra] += 1;
            }
        }

        for swap in &allowed_swaps {
            union(&mut parent, &mut rank, swap[0] as usize, swap[1] as usize);
        }

        // Group source values by root
        let mut groups: HashMap<usize, HashMap<i32, i32>> = HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            *groups.entry(root).or_default().entry(source[i]).or_insert(0) += 1;
        }

        let mut dist = 0;
        for i in 0..n {
            let root = find(&mut parent, i);
            let counts = groups.get_mut(&root).unwrap();
            if let Some(c) = counts.get_mut(&target[i]) {
                if *c > 0 {
                    *c -= 1;
                    continue;
                }
            }
            dist += 1;
        }

        dist
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimize_hamming_distance_after_swap_operations::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_minimum_hamming_distance_1() {
        let source = [1, 2, 3, 4].to_vec();
        let target = [2, 1, 4, 5].to_vec();
        let allowed_swaps = to_vec2d([[0, 1], [2, 3]]);
        assert_eq!(
            1,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );
    }

    #[test]
    fn test_minimum_hamming_distance_2() {
        let source = [1, 2, 3, 4].to_vec();
        let target = [1, 3, 2, 4].to_vec();
        let allowed_swaps = to_vec2d([[0; 0]; 0]);
        assert_eq!(
            2,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );
    }

    #[test]
    fn test_minimum_hamming_distance_3() {
        let source = [5, 1, 2, 4, 3].to_vec();
        let target = [1, 5, 4, 2, 3].to_vec();
        let allowed_swaps = to_vec2d([[0, 4], [4, 2], [1, 3], [1, 4]]);
        assert_eq!(
            0,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );
    }
}
