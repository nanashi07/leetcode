// # 2106. Maximum Fruits Harvested After at Most K Steps
// https://leetcode.com/problems/maximum-fruits-harvested-after-at-most-k-steps/description/

struct Solution;

impl Solution {
    pub fn max_total_fruits(fruits: Vec<Vec<i32>>, start_pos: i32, k: i32) -> i32 {
        todo!()
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
