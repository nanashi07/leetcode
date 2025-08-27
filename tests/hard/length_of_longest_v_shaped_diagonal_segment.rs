// # 3459. Length of Longest V-Shaped Diagonal Segment
// https://leetcode.com/problems/length-of-longest-v-shaped-diagonal-segment/

struct Solution;

impl Solution {
    pub fn len_of_v_diagonal(grid: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::length_of_longest_v_shaped_diagonal_segment::Solution;

    #[test]
    fn test_len_of_v_diagonal_1() {
        let grid = [
            [2, 2, 1, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<i32>>>();
        assert_eq!(5, Solution::len_of_v_diagonal(grid));
    }

    #[test]
    fn test_len_of_v_diagonal_2() {
        let grid = [
            [2, 2, 2, 2, 2],
            [2, 0, 2, 2, 0],
            [2, 0, 1, 1, 0],
            [1, 0, 2, 2, 2],
            [2, 0, 0, 2, 2],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<i32>>>();
        assert_eq!(4, Solution::len_of_v_diagonal(grid));
    }

    #[test]
    fn test_len_of_v_diagonal_3() {
        let grid = [
            [1, 2, 2, 2, 2],
            [2, 2, 2, 2, 0],
            [2, 0, 0, 0, 0],
            [0, 0, 2, 2, 2],
            [2, 0, 0, 2, 0],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<i32>>>();
        assert_eq!(5, Solution::len_of_v_diagonal(grid));
    }

    #[test]
    fn test_len_of_v_diagonal_4() {
        let grid = [[1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(1, Solution::len_of_v_diagonal(grid));
    }
}
