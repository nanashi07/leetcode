// 1189. Maximum Number of Balloons
// https://leetcode.com/problems/maximum-number-of-balloons/

struct Solution;

impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::maximum_number_of_balloons::Solution;

    #[test]
    fn test_max_number_of_balloons_1() {
        let text = "nlaebolko".to_string();
        assert_eq!(1, Solution::max_number_of_balloons(text));
    }

    #[test]
    fn test_max_number_of_balloons_2() {
        let text = "loonbalxballpoon".to_string();
        assert_eq!(2, Solution::max_number_of_balloons(text));
    }

    #[test]
    fn test_max_number_of_balloons_3() {
        let text = "leetcode".to_string();
        assert_eq!(0, Solution::max_number_of_balloons(text));
    }
}
