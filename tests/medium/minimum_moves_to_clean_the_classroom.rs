// 3568. Minimum Moves to Clean the Classroom
// https://leetcode.com/problems/minimum-moves-to-clean-the-classroom/

struct Solution;

impl Solution {
    pub fn min_moves(classroom: Vec<String>, energy: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_moves_to_clean_the_classroom::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_min_moves_1() {
        let classroom = to_string_vec(["S.", "XL"]);
        let energy = 2;
        assert_eq!(2, Solution::min_moves(classroom, energy));
    }

    #[test]
    fn test_min_moves_2() {
        let classroom = to_string_vec(["LS", "RL"]);
        let energy = 4;
        assert_eq!(3, Solution::min_moves(classroom, energy));
    }

    #[test]
    fn test_min_moves_3() {
        let classroom = to_string_vec(["L.S", "RXL"]);
        let energy = 3;
        assert_eq!(-1, Solution::min_moves(classroom, energy));
    }
}
