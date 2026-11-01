// 453. Minimum Moves to Equal Array Elements
// https://leetcode.com/problems/minimum-moves-to-equal-array-elements/

struct Solution;

impl Solution {
    // Incrementing n - 1 elements by 1 is equivalent to decrementing a single element
    // by 1, so every move brings one element closer to the minimum. The answer is
    // therefore sum(nums) - min(nums) * len(nums).
    pub fn min_moves(nums: Vec<i32>) -> i32 {
        let count = nums.len() as i64;
        let smallest = nums.iter().copied().min().unwrap_or(0);
        let sum: i64 = nums.into_iter().map(i64::from).sum();

        (sum - i64::from(smallest) * count) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_moves_to_equal_array_elements::Solution;

    #[test]
    fn test_min_moves_1() {
        let nums = [1, 2, 3].to_vec();
        assert_eq!(3, Solution::min_moves(nums.to_vec()));
    }

    #[test]
    fn test_min_moves_2() {
        let nums = [1, 1, 1].to_vec();
        assert_eq!(0, Solution::min_moves(nums.to_vec()));
    }
}
