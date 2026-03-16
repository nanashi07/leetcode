// 1878. Get Biggest Three Rhombus Sums in a Grid
// https://leetcode.com/problems/get-biggest-three-rhombus-sums-in-a-grid/

struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::get_biggest_three_rhombus_sums_in_a_grid::Solution;

    #[test]
    fn test_get_biggest_three_1() {
        let grid = [
            [3, 4, 5, 1, 3],
            [3, 3, 4, 2, 3],
            [20, 30, 200, 40, 10],
            [1, 5, 5, 4, 1],
            [4, 3, 2, 2, 5],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!([228, 216, 211].to_vec(), Solution::get_biggest_three(grid));
    }

    #[test]
    fn test_get_biggest_three_2() {
        let grid = [[1, 2, 3], [4, 5, 6], [7, 8, 9]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!([20, 9, 8].to_vec(), Solution::get_biggest_three(grid));
    }

    #[test]
    fn test_get_biggest_three_3() {
        let grid = [[7, 7, 7]].iter().map(|l| l.to_vec()).collect::<Vec<_>>();
        assert_eq!([7].to_vec(), Solution::get_biggest_three(grid));
    }
}
