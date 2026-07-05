// 1301. Number of Paths with Max Score
// https://leetcode.com/problems/number-of-paths-with-max-score/

struct Solution;

impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_paths_with_max_score::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_paths_with_max_score_1() {
        let board = to_string_vec(["E23", "2X2", "12S"]);
        assert_eq!([7, 1].to_vec(), Solution::paths_with_max_score(board));
    }

    #[test]
    fn test_paths_with_max_score_2() {
        let board = to_string_vec(["E12", "1X1", "21S"]);
        assert_eq!([4, 2].to_vec(), Solution::paths_with_max_score(board));
    }

    #[test]
    fn test_paths_with_max_score_3() {
        let board = to_string_vec(["E11", "XXX", "11S"]);
        assert_eq!([0, 0].to_vec(), Solution::paths_with_max_score(board));
    }
}
