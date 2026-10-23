// 3644. Maximum K to Sort a Permutation
// https://leetcode.com/problems/maximum-k-to-sort-a-permutation/

struct Solution;

impl Solution {
    // `a & b == k` forces every swapped value to contain all bits of `k`, so the values containing
    // `k` form a single swappable component (value `k` itself ANDs to exactly `k` with each of
    // them) while every other value is frozen in place. Sorting therefore needs each misplaced
    // index and its value to share all bits of `k`, i.e. `k` is a submask of every `i & nums[i]`;
    // the AND of those terms is itself the largest such `k` (0 when nothing is misplaced).
    pub fn sort_permutation(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .enumerate()
            .filter(|(i, v)| *v != *i as i32)
            .map(|(i, v)| v & (i as i32))
            .reduce(|a, b| a & b)
            .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_k_to_sort_a_permutation::Solution;

    #[test]
    fn test_sort_permutation_1() {
        let nums = [0, 3, 2, 1].to_vec();
        assert_eq!(1, Solution::sort_permutation(nums));
    }

    #[test]
    fn test_sort_permutation_2() {
        let nums = [0, 1, 3, 2].to_vec();
        assert_eq!(2, Solution::sort_permutation(nums));
    }

    #[test]
    fn test_sort_permutation_3() {
        let nums = [3, 2, 1, 0].to_vec();
        assert_eq!(0, Solution::sort_permutation(nums));
    }
}
