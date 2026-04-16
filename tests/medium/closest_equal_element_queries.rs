// 3488. Closest Equal Element Queries
// https://leetcode.com/problems/closest-equal-element-queries/

struct Solution;

impl Solution {
    pub fn solve_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        use std::collections::HashMap;

        let n = nums.len();
        // Group indices by value
        let mut indices: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            indices.entry(v).or_default().push(i);
        }

        queries
            .iter()
            .map(|&q| {
                let q = q as usize;
                let val = nums[q];
                let idx_list = &indices[&val];
                if idx_list.len() < 2 {
                    return -1;
                }
                // Binary search for q in the sorted index list
                let pos = idx_list.partition_point(|&x| x < q);
                let len = idx_list.len();
                // pos should point to q; check circular prev and next
                let prev = idx_list[(pos + len - 1) % len];
                let next = idx_list[(pos + 1) % len];
                let mut min_dist = i32::MAX;
                for &neighbor in &[prev, next] {
                    let diff = (q as i32 - neighbor as i32).unsigned_abs() as usize;
                    let circ = diff.min(n - diff);
                    min_dist = min_dist.min(circ as i32);
                }
                min_dist
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::closest_equal_element_queries::Solution;

    #[test]
    fn test_solve_queries_1() {
        let nums = [1, 3, 1, 4, 1, 3, 2].to_vec();
        let queries = [0, 3, 5].to_vec();
        let output = [2, -1, 3].to_vec();
        assert_eq!(output, Solution::solve_queries(nums, queries));
    }

    #[test]
    fn test_solve_queries_2() {
        let nums = [1, 2, 3, 4].to_vec();
        let queries = [0, 1, 2, 3].to_vec();
        let output = [-1, -1, -1, -1].to_vec();
        assert_eq!(output, Solution::solve_queries(nums, queries));
    }
}
