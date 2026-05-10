// 2770. Maximum Number of Jumps to Reach the Last Index
// https://leetcode.com/problems/maximum-number-of-jumps-to-reach-the-last-index/

struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        if n == 0 {
            return -1;
        }

        // Coordinate-compress values to support range-maximum queries by value.
        let mut vals = nums.clone();
        vals.sort_unstable();
        vals.dedup();

        let m = vals.len();
        let mut seg = vec![-1_000_000_000; 4 * m.max(1)];

        fn query(seg: &[i32], idx: usize, l: usize, r: usize, ql: usize, qr: usize) -> i32 {
            if ql <= l && r <= qr {
                return seg[idx];
            }
            let mid = (l + r) / 2;
            let mut ans = -1_000_000_000;
            if ql <= mid {
                ans = ans.max(query(seg, idx * 2, l, mid, ql, qr));
            }
            if mid < qr {
                ans = ans.max(query(seg, idx * 2 + 1, mid + 1, r, ql, qr));
            }
            ans
        }

        fn update(seg: &mut [i32], idx: usize, l: usize, r: usize, pos: usize, val: i32) {
            if l == r {
                seg[idx] = seg[idx].max(val);
                return;
            }
            let mid = (l + r) / 2;
            if pos <= mid {
                update(seg, idx * 2, l, mid, pos, val);
            } else {
                update(seg, idx * 2 + 1, mid + 1, r, pos, val);
            }
            seg[idx] = seg[idx * 2].max(seg[idx * 2 + 1]);
        }

        let mut dp = vec![-1_000_000_000; n];
        dp[0] = 0;
        let start_pos = vals.binary_search(&nums[0]).unwrap();
        update(&mut seg, 1, 0, m - 1, start_pos, 0);

        for i in 1..n {
            let lo = nums[i] - target;
            let hi = nums[i] + target;

            let left = vals.partition_point(|&v| v < lo);
            let right_exclusive = vals.partition_point(|&v| v <= hi);

            if left < right_exclusive {
                let best = query(&seg, 1, 0, m - 1, left, right_exclusive - 1);
                if best > -1_000_000_000 {
                    dp[i] = best + 1;
                }
            }

            if dp[i] > -1_000_000_000 {
                let pos = vals.binary_search(&nums[i]).unwrap();
                update(&mut seg, 1, 0, m - 1, pos, dp[i]);
            }
        }

        if dp[n - 1] < 0 {
            -1
        } else {
            dp[n - 1]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_jumps_to_reach_the_last_index::Solution;

    #[test]
    fn test_maximum_jumps_1() {
        let nums = [1, 3, 6, 4, 1, 2].to_vec();
        let target = 2;
        assert_eq!(3, Solution::maximum_jumps(nums, target));
    }

    #[test]
    fn test_maximum_jumps_2() {
        let nums = [1, 3, 6, 4, 1, 2].to_vec();
        let target = 3;
        assert_eq!(5, Solution::maximum_jumps(nums, target));
    }

    #[test]
    fn test_maximum_jumps_3() {
        let nums = [1, 3, 6, 4, 1, 2].to_vec();
        let target = 0;
        assert_eq!(-1, Solution::maximum_jumps(nums, target));
    }

    #[test]
    fn test_maximum_jumps_4() {
        let nums = [
            758043978, 79060681, 785252849, 287889790, -983845055, 224430896, -477101480,
        ]
        .to_vec();
        let target = 1769097904;
        assert_eq!(6, Solution::maximum_jumps(nums, target));
    }
}
