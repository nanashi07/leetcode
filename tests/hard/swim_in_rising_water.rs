// # 778. Swim in Rising Water
// https://leetcode.com/problems/swim-in-rising-water/

struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::swim_in_rising_water::Solution;

    #[test]
    fn test_swim_in_water_1() {
        let grid = [[0, 2], [1, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::swim_in_water(grid));
    }

    #[test]
    fn test_swim_in_water_2() {
        let grid = [
            [0, 1, 2, 3, 4],
            [24, 23, 22, 21, 5],
            [12, 13, 14, 15, 16],
            [11, 17, 18, 19, 20],
            [10, 9, 8, 7, 6],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<_>>();
        assert_eq!(16, Solution::swim_in_water(grid));
    }
}
