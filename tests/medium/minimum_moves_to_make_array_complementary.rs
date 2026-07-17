// 1674. Minimum Moves to Make Array Complementary
// https://leetcode.com/problems/minimum-moves-to-make-array-complementary/

struct Solution;

impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let n = nums.len();
        let pair_count = n / 2;
        let max_sum = (2 * limit) as usize;
        let mut diff = vec![0; max_sum + 2];

        for i in 0..pair_count {
            let a = nums[i];
            let b = nums[n - 1 - i];
            let low = (a.min(b) + 1) as usize;
            let high = (a.max(b) + limit) as usize;
            let sum = (a + b) as usize;

            diff[2] += 2;
            diff[low] -= 1;
            diff[sum] -= 1;
            diff[sum + 1] += 1;
            diff[high + 1] += 1;
        }

        let mut best = i32::MAX;
        let mut current = 0;

        for d in diff.iter().take(max_sum + 1).skip(2) {
            current += *d;
            best = best.min(current);
        }

        best
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_moves_to_make_array_complementary::Solution;

    #[test]
    fn test_min_moves_1() {
        let nums = [1, 2, 4, 3].to_vec();
        let limit = 4;
        assert_eq!(1, Solution::min_moves(nums, limit));
    }

    #[test]
    fn test_min_moves_2() {
        let nums = [1, 2, 2, 1].to_vec();
        let limit = 2;
        assert_eq!(2, Solution::min_moves(nums, limit));
    }

    #[test]
    fn test_min_moves_3() {
        let nums = [1, 2, 1, 2].to_vec();
        let limit = 2;
        assert_eq!(0, Solution::min_moves(nums, limit));
    }
}
