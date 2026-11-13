// 3420. Count Non-Decreasing Subarrays After K Operations
// https://leetcode.com/problems/count-non-decreasing-subarrays-after-k-operations/

struct Solution;

impl Solution {
    pub fn count_non_decreasing_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        if n == 0 {
            return 0;
        }

        // An element can only be raised, so in a non-decreasing result it must end
        // up at least as large as every earlier element of the subarray: the cost
        // is sum(running maximum - value). `next[i]` is the first index after `i`
        // holding a strictly larger value, i.e. where that running maximum changes.
        let mut next = vec![n; n + 1];
        let mut stack: Vec<usize> = Vec::with_capacity(n);
        for i in (0..n).rev() {
            while stack.last().is_some_and(|&top| nums[top] <= nums[i]) {
                stack.pop();
            }
            next[i] = stack.last().copied().unwrap_or(n);
            stack.push(i);
        }

        let mut val: Vec<i64> = nums.iter().map(|&x| i64::from(x)).collect();
        val.push(0); // sentinel value for index `n`
        let mut pre = vec![0i64; n + 1];
        for (i, &v) in val.iter().enumerate().take(n) {
            pre[i + 1] = pre[i] + v;
        }

        // Binary lifting along the "next greater" chain: `jump[t][i]` is the node
        // reached after 2^t links and `gain[t][i]` sums val[c] * (next[c] - c) over
        // those nodes, which is each node's whole share of a prefix-maximum sum.
        let log = usize::try_from((n + 1).ilog2()).unwrap_or(0) + 2;
        let mut jump = vec![vec![0usize; n + 1]; log];
        let mut gain = vec![vec![0i64; n + 1]; log];
        for i in 0..=n {
            jump[0][i] = next[i];
            gain[0][i] = val[i] * (next[i] - i) as i64;
        }
        for t in 1..log {
            let (prev_jump, cur_jump) = jump.split_at_mut(t);
            let (prev_gain, cur_gain) = gain.split_at_mut(t);
            let (prev_jump, prev_gain) = (&prev_jump[t - 1], &prev_gain[t - 1]);
            for i in 0..=n {
                let mid = prev_jump[i];
                cur_jump[0][i] = prev_jump[mid];
                cur_gain[0][i] = prev_gain[i] + prev_gain[mid];
            }
        }

        // Sum of running maxima over nums[l..=r], walking only the chain nodes that
        // actually raise the maximum; every other position inherits its segment.
        let cost = |l: usize, r: usize| -> i64 {
            let end = r + 1;
            let mut cur = l;
            let mut total = 0i64;
            for t in (0..log).rev() {
                if jump[t][cur] <= end {
                    total += gain[t][cur];
                    cur = jump[t][cur];
                }
            }
            // `cur` is the last chain node inside [l, r]; its segment stops at `end`.
            let max_sum = total + val[cur] * (end - cur) as i64;
            max_sum - (pre[r + 1] - pre[l])
        };

        // The cost of a subarray only grows when it is extended, so one sliding
        // window over the right endpoint counts every affordable start at once.
        let budget = i64::from(k);
        let mut ans = 0i64;
        let mut left = 0usize;
        for right in 0..n {
            while cost(left, right) > budget {
                left += 1;
            }
            ans += (right - left + 1) as i64;
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::count_non_decreasing_subarrays_after_k_operations::Solution;

    #[test]
    fn test_count_non_decreasing_subarrays_1() {
        let nums: Vec<i32> = vec![6, 3, 1, 2, 4, 4];
        let k: i32 = 7;
        assert_eq!(17, Solution::count_non_decreasing_subarrays(nums, k));
    }

    #[test]
    fn test_count_non_decreasing_subarrays_2() {
        let nums: Vec<i32> = vec![6, 3, 1, 3, 6];
        let k: i32 = 4;
        assert_eq!(12, Solution::count_non_decreasing_subarrays(nums, k));
    }
}
