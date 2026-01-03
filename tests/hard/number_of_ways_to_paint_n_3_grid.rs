// 1411. Number of Ways to Paint N × 3 Grid
// https://leetcode.com/problems/number-of-ways-to-paint-n-3-grid/

struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_ways_to_paint_n_3_grid::Solution;

    #[test]
    fn test_num_of_ways_1() {
        let n = 1;
        assert_eq!(12, Solution::num_of_ways(n));
    }

    #[test]
    fn test_num_of_ways_2() {
        let n = 5000;
        assert_eq!(30228214, Solution::num_of_ways(n));
    }
}
