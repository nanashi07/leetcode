// # 3000. Maximum Area of Longest Diagonal Rectangle
// https://leetcode.com/problems/maximum-area-of-longest-diagonal-rectangle/

struct Solution;

impl Solution {
    pub fn area_of_max_diagonal(dimensions: Vec<Vec<i32>>) -> i32 {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_area_of_longest_diagonal_rectangle::Solution;

    #[test]
    fn test_area_of_max_diagonal_1() {
        let dimensions = [[9, 3], [8, 6]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(48, Solution::area_of_max_diagonal(dimensions));
    }

    #[test]
    fn test_area_of_max_diagonal_2() {
        let dimensions = [[3, 4], [4, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(12, Solution::area_of_max_diagonal(dimensions));
    }
}
