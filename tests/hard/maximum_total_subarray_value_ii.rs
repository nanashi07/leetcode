// 3691. Maximum Total Subarray Value II
// https://leetcode.com/problems/maximum-total-subarray-value-ii/

struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let mut lg = vec![0usize; n + 1];
        for i in 2..=n {
            lg[i] = lg[i >> 1] + 1;
        }

        let mut max_log = 1usize;
        while (1usize << max_log) <= n {
            max_log += 1;
        }

        let mut st_max = vec![vec![0i32; max_log]; n];
        let mut st_min = vec![vec![0i32; max_log]; n];
        for i in 0..n {
            st_max[i][0] = nums[i];
            st_min[i][0] = nums[i];
        }

        for j in 1..max_log {
            let len = 1usize << j;
            let half = len >> 1;
            for i in 0..=n - len {
                st_max[i][j] = st_max[i][j - 1].max(st_max[i + half][j - 1]);
                st_min[i][j] = st_min[i][j - 1].min(st_min[i + half][j - 1]);
            }
        }

        let value = |l: usize, r: usize, st_max: &Vec<Vec<i32>>, st_min: &Vec<Vec<i32>>, lg: &Vec<usize>| {
            let p = lg[r - l + 1];
            let len = 1usize << p;
            let mx = st_max[l][p].max(st_max[r + 1 - len][p]);
            let mn = st_min[l][p].min(st_min[r + 1 - len][p]);
            mx as i64 - mn as i64
        };

        let mut heap = std::collections::BinaryHeap::new();
        for l in 0..n {
            heap.push((value(l, n - 1, &st_max, &st_min, &lg), l, n - 1));
        }

        let mut ans = 0i64;
        for _ in 0..k as usize {
            let (val, l, r) = heap.pop().unwrap();
            ans += val;
            if r > l {
                heap.push((value(l, r - 1, &st_max, &st_min, &lg), l, r - 1));
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_total_subarray_value_ii::Solution;

    #[test]
    fn test_max_total_value_1() {
        let nums = [1, 3, 2].to_vec();
        let k = 2;
        assert_eq!(4, Solution::max_total_value(nums, k));
    }

    #[test]
    fn test_max_total_value_2() {
        let nums = [4, 2, 5, 1].to_vec();
        let k = 3;
        assert_eq!(12, Solution::max_total_value(nums, k));
    }
}
