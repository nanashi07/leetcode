// 657. Robot Return to Origin
// https://leetcode.com/problems/robot-return-to-origin/

struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::robot_return_to_origin::Solution;

    #[test]
    fn test_judge_circle_1() {
        let moves = "UD".to_string();
        assert_eq!(true, Solution::judge_circle(moves));
    }

    #[test]
    fn test_judge_circle_2() {
        let moves = "LL".to_string();
        assert_eq!(false, Solution::judge_circle(moves));
    }
}
