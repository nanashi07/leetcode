// # 3197. Find the Minimum Area to Cover All Ones II
// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-ii/description/?envType=daily-question&envId=2025-08-23

struct Solution;

impl Solution {
    pub fn minimum_sum(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_minimum_area_to_cover_all_ones_ii::Solution;

    #[test]
    fn test_minimum_sum_1() {
        let grid = [[1, 0, 1], [1, 1, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(5, Solution::minimum_sum(grid));
    }

    #[test]
    fn test_minimum_sum_2() {
        let grid = [[1, 0, 1, 0], [0, 1, 0, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(5, Solution::minimum_sum(grid));
    }
}
