// 3934. Smallest Unique Subarray
// https://leetcode.com/problems/smallest-unique-subarray/

struct Solution;

impl Solution {
    /// A subarray starting at `p` is unique exactly when its length exceeds the
    /// longest common prefix it shares with any other suffix of `nums`. In
    /// suffix-array order that maximum equals `max(lcp_prev, lcp_next)`, so the
    /// shortest unique subarray starting at `p` has length `lcp_max + 1`.
    ///
    /// Time: O(n log n), Space: O(n)
    pub fn smallest_unique_subarray(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n == 1 {
            return 1;
        }

        let sa = Self::suffix_array(&nums);

        // rank[pos] = index of the suffix starting at pos inside `sa`
        let mut rank = vec![0usize; n];
        for (r, &pos) in sa.iter().enumerate() {
            rank[pos] = r;
        }

        // Kasai: lcp[r] = LCP(sa[r], sa[r - 1]), lcp[0] stays unused
        let mut lcp = vec![0usize; n];
        let mut h = 0usize;
        for pos in 0..n {
            let r = rank[pos];
            if r == 0 {
                h = 0;
                continue;
            }
            let other = sa[r - 1];
            while pos + h < n && other + h < n && nums[pos + h] == nums[other + h] {
                h += 1;
            }
            lcp[r] = h;
            h = h.saturating_sub(1);
        }

        let mut best = n;
        for r in 0..n {
            let shared = lcp[r].max(if r + 1 < n { lcp[r + 1] } else { 0 });
            let pos = sa[r];
            // `shared` can only equal the whole remaining suffix when that
            // suffix is a prefix of another one; then nothing starting here is unique.
            if shared < n - pos {
                best = best.min(shared + 1);
            }
        }

        i32::try_from(best).unwrap_or(i32::MAX)
    }

    /// Suffix array of an integer alphabet via prefix doubling.
    fn suffix_array(s: &[i32]) -> Vec<usize> {
        let n = s.len();

        // Coordinate-compress the alphabet into 1..=classes (0 is reserved as the
        // "end of string" sentinel).
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by_key(|&i| s[i]);
        let mut rank = vec![0usize; n];
        let mut classes = 0usize;
        for (k, &pos) in order.iter().enumerate() {
            if k > 0 && s[pos] != s[order[k - 1]] {
                classes += 1;
            }
            rank[pos] = classes + 1;
        }

        let mut sa: Vec<usize> = (0..n).collect();
        let mut next_rank = vec![0usize; n];
        let mut len = 1usize;
        loop {
            sa.sort_unstable_by_key(|&pos| {
                (rank[pos], if pos + len < n { rank[pos + len] } else { 0 })
            });

            next_rank[sa[0]] = 1;
            let mut classes = 1usize;
            for w in 1..n {
                let (prev, cur) = (sa[w - 1], sa[w]);
                let prev_key = (
                    rank[prev],
                    if prev + len < n { rank[prev + len] } else { 0 },
                );
                let cur_key = (rank[cur], if cur + len < n { rank[cur + len] } else { 0 });
                if prev_key != cur_key {
                    classes += 1;
                }
                next_rank[cur] = classes;
            }

            std::mem::swap(&mut rank, &mut next_rank);
            if classes == n {
                break;
            }
            len *= 2;
        }

        sa
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::smallest_unique_subarray::Solution;

    #[test]
    fn test_smallest_unique_subarray_1() {
        let nums: Vec<i32> = vec![3, 3, 3];
        assert_eq!(3, Solution::smallest_unique_subarray(nums));
    }

    #[test]
    fn test_smallest_unique_subarray_2() {
        let nums: Vec<i32> = vec![2, 1, 2, 3, 3];
        assert_eq!(1, Solution::smallest_unique_subarray(nums));
    }

    #[test]
    fn test_smallest_unique_subarray_3() {
        let nums: Vec<i32> = vec![1, 1, 2, 2, 1];
        assert_eq!(2, Solution::smallest_unique_subarray(nums));
    }
}
