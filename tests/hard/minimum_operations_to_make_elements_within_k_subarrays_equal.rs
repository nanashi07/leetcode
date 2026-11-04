// 3505. Minimum Operations to Make Elements Within K Subarrays Equal
// https://leetcode.com/problems/minimum-operations-to-make-elements-within-k-subarrays-equal/

struct Solution;

const INF: i64 = i64::MAX / 4;

/// Lowest set bit of `i`, i.e. `i & -i`. Wrapping math is used because `usize` has no
/// `Neg` impl and `isolate_lowest_one` needs a newer toolchain than this repo targets.
fn lowbit(i: usize) -> usize {
    (!i).wrapping_add(1) & i
}

fn bit_add(bit: &mut [i64], mut i: usize, delta: i64) {
    while i < bit.len() {
        bit[i] += delta;
        i += lowbit(i);
    }
}

fn bit_prefix(bit: &[i64], mut i: usize) -> i64 {
    let mut acc = 0i64;
    while i > 0 {
        acc += bit[i];
        i -= lowbit(i);
    }
    acc
}

/// Smallest 1-based position whose prefix count reaches `rank`, plus the number of
/// elements stored strictly before that position.
fn bit_kth(bit: &[i64], rank: i64) -> (usize, i64) {
    let n = bit.len() - 1;
    let mut step = 1usize;
    while step * 2 <= n {
        step *= 2;
    }

    let mut idx = 0usize;
    let mut before = 0i64;
    while step > 0 {
        let next = idx + step;
        if next <= n && before + bit[next] < rank {
            idx = next;
            before += bit[next];
        }
        step /= 2;
    }

    (idx + 1, before)
}

impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32, k: i32) -> i64 {
        let n = nums.len();
        let len = x as usize;
        let groups = k as usize;
        let rank = len.div_ceil(2) as i64;

        // Coordinate-compressed value domain for the two Fenwick trees.
        let mut vals: Vec<i64> = nums.iter().map(|&v| v as i64).collect();
        vals.sort_unstable();
        vals.dedup();
        let pos = |v: i64| vals.partition_point(|&u| u < v) + 1;

        // cost[s] = operations that make every element of nums[s..s + len] equal,
        // which is the sum of distances to the window median.
        let mut cnt = vec![0i64; vals.len() + 1];
        let mut sum = vec![0i64; vals.len() + 1];
        let mut window_sum = 0i64;
        let mut cost: Vec<i64> = Vec::with_capacity(n - len + 1);

        for (i, &v) in nums.iter().enumerate() {
            let v = i64::from(v);
            let p = pos(v);
            bit_add(&mut cnt, p, 1);
            bit_add(&mut sum, p, v);
            window_sum += v;

            if i >= len {
                let old = i64::from(nums[i - len]);
                let q = pos(old);
                bit_add(&mut cnt, q, -1);
                bit_add(&mut sum, q, -old);
                window_sum -= old;
            }

            if i + 1 >= len {
                let (mid, left_cnt) = bit_kth(&cnt, rank);
                let median = vals[mid - 1];
                let left_sum = bit_prefix(&sum, mid - 1);
                let right_cnt = len as i64 - bit_prefix(&cnt, mid);
                let right_sum = window_sum - bit_prefix(&sum, mid);
                cost.push(median * left_cnt - left_sum + right_sum - median * right_cnt);
            }
        }

        // best[j][i] = cheapest way to place j groups inside nums[..i]; only the previous
        // row is needed. A group taken at i - len frees everything before it for j - 1.
        let mut prev = vec![0i64; n + 1];
        for _ in 0..groups {
            let mut best = vec![INF; n + 1];
            for i in 1..=n {
                best[i] = best[i - 1];
                if i >= len && prev[i - len] < INF {
                    let take = prev[i - len] + cost[i - len];
                    if take < best[i] {
                        best[i] = take;
                    }
                }
            }
            prev = best;
        }

        prev[n]
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_operations_to_make_elements_within_k_subarrays_equal::Solution;

    #[test]
    fn test_min_operations_1() {
        let nums = [5, -2, 1, 3, 7, 3, 6, 4, -1].to_vec();
        let x = 3;
        let k = 2;
        assert_eq!(8, Solution::min_operations(nums, x, k));
    }

    #[test]
    fn test_min_operations_2() {
        let nums = [9, -2, -2, -2, 1, 5].to_vec();
        let x = 2;
        let k = 2;
        assert_eq!(3, Solution::min_operations(nums, x, k));
    }
}
