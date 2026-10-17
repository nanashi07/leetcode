// 3551. Minimum Swaps to Sort by Digit Sum
// https://leetcode.com/problems/minimum-swaps-to-sort-by-digit-sum/

struct Solution;

impl Solution {
    pub fn min_swaps(nums: Vec<i32>) -> i32 {
        let mut ordered: Vec<(i32, i32, usize)> = nums
            .iter()
            .enumerate()
            .map(|(index, &value)| {
                let mut remaining = value;
                let mut digit_sum = 0;
                while remaining > 0 {
                    digit_sum += remaining % 10;
                    remaining /= 10;
                }
                (digit_sum, value, index)
            })
            .collect();
        ordered.sort_unstable();

        let mut visited = vec![false; nums.len()];
        let mut swaps: usize = 0;
        for start in 0..nums.len() {
            if visited[start] {
                continue;
            }

            let mut cycle_len = 0;
            let mut index = start;
            while !visited[index] {
                visited[index] = true;
                index = ordered[index].2;
                cycle_len += 1;
            }
            swaps += cycle_len - 1;
        }

        swaps as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_swaps_to_sort_by_digit_sum::Solution;

    #[test]
    fn test_min_swaps_1() {
        let nums = [37, 100].to_vec();
        assert_eq!(1, Solution::min_swaps(nums));
    }

    #[test]
    fn test_min_swaps_2() {
        let nums = [22, 14, 33, 7].to_vec();
        assert_eq!(0, Solution::min_swaps(nums));
    }

    #[test]
    fn test_min_swaps_3() {
        let nums = [18, 43, 34, 16].to_vec();
        assert_eq!(2, Solution::min_swaps(nums));
    }
}
