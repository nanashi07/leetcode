// 1970. Last Day Where You Can Still Cross
// https://leetcode.com/problems/last-day-where-you-can-still-cross/

struct Solution;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::last_day_where_you_can_still_cross::Solution;

    #[test]
    fn test_latest_day_to_cross_1() {
        let row = 2;
        let col = 2;
        let cells = [[1, 1], [2, 1], [1, 2], [2, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(2, Solution::latest_day_to_cross(row, col, cells));
    }

    #[test]
    fn test_latest_day_to_cross_2() {
        let row = 2;
        let col = 2;
        let cells = [[1, 1], [1, 2], [2, 1], [2, 2]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(1, Solution::latest_day_to_cross(row, col, cells));
    }

    #[test]
    fn test_latest_day_to_cross_3() {
        let row = 3;
        let col = 3;
        let cells = [
            [1, 2],
            [2, 1],
            [3, 3],
            [2, 2],
            [1, 1],
            [1, 3],
            [2, 3],
            [3, 2],
            [3, 1],
        ]
        .iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(3, Solution::latest_day_to_cross(row, col, cells));
    }
}
