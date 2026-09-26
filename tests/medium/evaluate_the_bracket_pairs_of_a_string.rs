// 1807. Evaluate the Bracket Pairs of a String
// https://leetcode.com/problems/evaluate-the-bracket-pairs-of-a-string/

struct Solution;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::evaluate_the_bracket_pairs_of_a_string::Solution;
    use crate::shared::vec2d::{to_string_vec, to_string_vec2d};

    #[test]
    fn test_evaluate_1() {
        let s = "(name)is(age)yearsold".to_string();
        let knowledge = to_string_vec2d([["name", "bob"], ["age", "two"]]);
        assert_eq!(
            "bobistwoyearsold".to_string(),
            Solution::evaluate(s, knowledge)
        );
    }

    #[test]
    fn test_evaluate_2() {
        let s = "hi(name)".to_string();
        let knowledge = to_string_vec2d([["a", "b"]]);
        assert_eq!("hi?".to_string(), Solution::evaluate(s, knowledge));
    }

    #[test]
    fn test_evaluate_3() {
        let s = "(a)(a)(a)aaa".to_string();
        let knowledge = to_string_vec2d([["a", "yes"]]);
        assert_eq!("yesyesyesaaa".to_string(), Solution::evaluate(s, knowledge));
    }
}
