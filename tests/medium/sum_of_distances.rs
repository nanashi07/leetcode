// 2615. Sum of Distances
// https://leetcode.com/problems/sum-of-distances/

struct Solution;

impl Solution {
    pub fn distance(nums: Vec<i32>) -> Vec<i64> {
        use std::collections::HashMap;
        let n = nums.len();
        let mut result = vec![0i64; n];
        let mut groups: HashMap<i32, Vec<usize>> = HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            groups.entry(v).or_default().push(i);
        }
        for indices in groups.values() {
            let m = indices.len();
            // prefix sum of indices
            let mut prefix = vec![0i64; m + 1];
            for i in 0..m {
                prefix[i + 1] = prefix[i] + indices[i] as i64;
            }
            for (k, &idx) in indices.iter().enumerate() {
                let left = idx as i64 * k as i64 - prefix[k];
                let right = (prefix[m] - prefix[k + 1]) - idx as i64 * (m - k - 1) as i64;
                result[idx] = left + right;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sum_of_distances::Solution;

    #[test]
    fn test_distance_1() {
        let nums = [1, 3, 1, 1, 2].to_vec();
        assert_eq!([5, 0, 3, 4, 0].to_vec(), Solution::distance(nums));
    }

    #[test]
    fn test_distance_2() {
        let nums = [0, 5, 3].to_vec();
        assert_eq!([0, 0, 0].to_vec(), Solution::distance(nums));
    }
}
