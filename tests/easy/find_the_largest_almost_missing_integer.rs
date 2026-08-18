// 3471. Find the Largest Almost Missing Integer
// https://leetcode.com/problems/find-the-largest-almost-missing-integer/

struct Solution;

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let k = k as usize;
        // Track first and last occurrence index for each value.
        // A value appears in exactly 1 window of size k iff
        // min(last, n-k) == max(first+1, k) - 1, i.e. window count == 1.
        let mut first: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        let mut last: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        for (i, &v) in nums.iter().enumerate() {
            first.entry(v).or_insert(i);
            last.insert(v, i);
        }
        first
            .iter()
            .filter(|(v, fi)| {
                let li = last[&v];
                // windows containing v: [fi-k+1, li] clamped to [0, n-k]
                // count = min(li, n-k) - max(fi+1-k, 0) + 1 == 1
                li.min(n - k) == fi.saturating_sub(k - 1)
            })
            .map(|(&v, _)| v)
            .max()
            .unwrap_or(-1)
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_the_largest_almost_missing_integer::Solution;

    #[test]
    fn test_largest_integer_1() {
        let nums = [3, 9, 2, 1, 7].to_vec();
        let k = 3;
        assert_eq!(7, Solution::largest_integer(nums, k));
    }

    #[test]
    fn test_largest_integer_2() {
        let nums = [3, 9, 7, 2, 1, 7].to_vec();
        let k = 4;
        assert_eq!(3, Solution::largest_integer(nums, k));
    }

    #[test]
    fn test_largest_integer_3() {
        let nums = [0, 0].to_vec();
        let k = 1;
        assert_eq!(-1, Solution::largest_integer(nums, k));
    }
}
