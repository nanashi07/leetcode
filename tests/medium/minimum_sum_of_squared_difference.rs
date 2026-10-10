// 2333. Minimum Sum of Squared Difference
// https://leetcode.com/problems/minimum-sum-of-squared-difference/

struct Solution;

impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let budget = i64::from(k1) + i64::from(k2);
        let differences: Vec<i64> = nums1
            .into_iter()
            .zip(nums2)
            .map(|(a, b)| (i64::from(a) - i64::from(b)).abs())
            .collect();

        if differences.iter().sum::<i64>() <= budget {
            return 0;
        }

        let mut low = 0_i64;
        let mut high = differences.iter().copied().max().unwrap_or(0);
        while low < high {
            let middle = low + (high - low) / 2;
            let required = differences
                .iter()
                .map(|&difference| (difference - middle).max(0))
                .sum::<i64>();
            if required <= budget {
                high = middle;
            } else {
                low = middle + 1;
            }
        }

        let required = differences
            .iter()
            .map(|&difference| (difference - low).max(0))
            .sum::<i64>();
        let remaining = budget - required;
        differences
            .iter()
            .map(|&difference| difference.min(low).pow(2))
            .sum::<i64>()
            - remaining * (2 * low - 1)
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_sum_of_squared_difference::Solution;

    #[test]
    fn test_min_sum_square_diff_1() {
        let nums1 = [1, 2, 3, 4].to_vec();
        let nums2 = [2, 10, 20, 19].to_vec();
        let k1 = 0;
        let k2 = 0;
        assert_eq!(579, Solution::min_sum_square_diff(nums1, nums2, k1, k2));
    }

    #[test]
    fn test_min_sum_square_diff_2() {
        let nums1 = [1, 4, 10, 12].to_vec();
        let nums2 = [5, 8, 6, 9].to_vec();
        let k1 = 1;
        let k2 = 1;
        assert_eq!(43, Solution::min_sum_square_diff(nums1, nums2, k1, k2));
    }
}
