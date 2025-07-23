// # 1717. Maximum Score From Removing Substrings
// https://leetcode.com/problems/maximum-score-from-removing-substrings/

struct Solution;

impl Solution {
    pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_score_from_removing_substrings::Solution;

    #[test]
    fn test_maximum_gain_1() {
        let s = "cdbcbbaaabab".to_owned();
        let x = 4;
        let y = 5;
        assert_eq!(19, Solution::maximum_gain(s, x, y));
    }

    #[test]
    fn test_maximum_gain_2() {
        let s = "aabbaaxybbaabb".to_owned();
        let x = 5;
        let y = 4;
        assert_eq!(20, Solution::maximum_gain(s, x, y));
    }
}
