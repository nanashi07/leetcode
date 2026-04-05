// 657. Robot Return to Origin
// https://leetcode.com/problems/robot-return-to-origin/

struct Solution;

impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        println!("moves: {moves}");

        let mut h = 0;
        let mut v = 0;
        for c in moves.chars() {
            match c {
                'L' => h -= 1,
                'R' => h += 1,
                'U' => v += 1,
                'D' => v -= 1,
                _ => {}
            }
        }
        h == 0 && v == 0
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
