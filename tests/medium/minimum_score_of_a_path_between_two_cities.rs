// 2492. Minimum Score of a Path Between Two Cities
// https://leetcode.com/problems/minimum-score-of-a-path-between-two-cities/

struct Solution;

impl Solution {
    pub fn min_score(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut parent: Vec<usize> = (0..=n).collect();
        let mut rank = vec![0u8; n + 1];
        let mut min_weight = vec![i32::MAX; n + 1];

        fn find(parent: &mut [usize], i: usize) -> usize {
            let mut root = i;
            while root != parent[root] {
                root = parent[root];
            }
            let mut curr = i;
            while curr != root {
                let next = parent[curr];
                parent[curr] = root;
                curr = next;
            }
            root
        }

        fn union(
            parent: &mut [usize],
            rank: &mut [u8],
            min_weight: &mut [i32],
            i: usize,
            j: usize,
            weight: i32,
        ) {
            let root_i = find(parent, i);
            let root_j = find(parent, j);
            let new_min = min_weight[root_i].min(min_weight[root_j]).min(weight);
            if root_i != root_j {
                if rank[root_i] < rank[root_j] {
                    parent[root_i] = root_j;
                    min_weight[root_j] = new_min;
                } else if rank[root_i] > rank[root_j] {
                    parent[root_j] = root_i;
                    min_weight[root_i] = new_min;
                } else {
                    parent[root_i] = root_j;
                    rank[root_j] += 1;
                    min_weight[root_j] = new_min;
                }
            } else {
                min_weight[root_i] = new_min;
            }
        }

        for road in &roads {
            let u = road[0] as usize;
            let v = road[1] as usize;
            let w = road[2];
            union(&mut parent, &mut rank, &mut min_weight, u, v, w);
        }

        let root_1 = find(&mut parent, 1);
        min_weight[root_1]
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_score_of_a_path_between_two_cities::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_min_score_1() {
        let n = 4;
        let roads = to_vec2d([[1, 2, 9], [2, 3, 6], [2, 4, 5], [1, 4, 7]]);
        assert_eq!(5, Solution::min_score(n, roads));
    }

    #[test]
    fn test_min_score_2() {
        let n = 4;
        let roads = to_vec2d([[1, 2, 2], [1, 3, 4], [3, 4, 7]]);
        assert_eq!(2, Solution::min_score(n, roads));
    }
}
