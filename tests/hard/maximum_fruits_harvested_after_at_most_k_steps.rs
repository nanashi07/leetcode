// # 2106. Maximum Fruits Harvested After at Most K Steps
// https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps/

struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        let n = fruits.len();
        let mut sum = vec![0; n + 1];
        let mut indices = vec![0; n];

        // Build prefix sum and indices arrays
        for i in 0..n {
            sum[i + 1] = sum[i] + fruits[i][1];
            indices[i] = fruits[i][0];
        }

        let mut ans = 0;

        // Try all possible distributions of steps
        for x in 0..=k / 2 {
            // Move left x steps, then move right k - x steps
            let y = k - 2 * x;
            let left = start_pos - x;
            let right = start_pos + y;

            let start = indices.binary_search(&left).unwrap_or_else(|i| i);
            let end = indices.binary_search(&(right + 1)).unwrap_or_else(|i| i);

            ans = ans.max(sum[end] - sum[start]);

            // Move right x steps, then move left k - x steps
            let y = k - 2 * x;
            let left = start_pos - y;
            let right = start_pos + x;

            let start = indices.binary_search(&left).unwrap_or_else(|i| i);
            let end = indices.binary_search(&(right + 1)).unwrap_or_else(|i| i);

            ans = ans.max(sum[end] - sum[start]);
        }

        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_fruits_harvested_after_at_most_k_steps::Solution;

    #[test]
    fn test_max_total_fruits_1() {
        let fruits = [[2, 8], [6, 3], [8, 6]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let start_pos = 5;
        let k = 4;
        assert_eq!(9, Solution::max_total_fruits(fruits, start_pos, k));
    }

    #[test]
    fn test_max_total_fruits_2() {
        let fruits = [[0, 9], [4, 1], [5, 7], [6, 2], [7, 4], [10, 9]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let start_pos = 5;
        let k = 4;
        assert_eq!(14, Solution::max_total_fruits(fruits, start_pos, k));
    }

    #[test]
    fn test_max_total_fruits_3() {
        let fruits = [[0, 3], [6, 4], [8, 5]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let start_pos = 3;
        let k = 2;
        assert_eq!(0, Solution::max_total_fruits(fruits, start_pos, k));
    }
}
