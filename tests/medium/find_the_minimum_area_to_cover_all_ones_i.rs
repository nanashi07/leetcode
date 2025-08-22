// # 3195. Find the Minimum Area to Cover All Ones I
// https://leetcode.com/problems/find-the-minimum-area-to-cover-all-ones-i/

struct Solution;

impl Solution {
    pub fn minimum_area(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_minimum_area_to_cover_all_ones_i::Solution;

    #[test]
    fn test_minimum_area_1() {
        let grid = [[0, 1, 0], [1, 0, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(6, Solution::minimum_area(grid));
    }

    #[test]
    fn test_minimum_area_2() {
        let grid = [[1, 0], [0, 0]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(1, Solution::minimum_area(grid));
    }
}
