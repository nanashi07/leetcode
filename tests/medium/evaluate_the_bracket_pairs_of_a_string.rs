// 1807. Evaluate the Bracket Pairs of a String
// https://leetcode.com/problems/evaluate-the-bracket-pairs-of-a-string/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn evaluate(s: String, knowledge: Vec<Vec<String>>) -> String {
        let map: HashMap<&str, &str> = knowledge
            .iter()
            .map(|entry| (entry[0].as_str(), entry[1].as_str()))
            .collect();

        let bytes = s.as_bytes();
        let mut result = String::with_capacity(s.len());
        let mut i = 0;

        while i < bytes.len() {
            if bytes[i] != b'(' {
                // `s` holds only lowercase letters and ASCII parentheses, so a run of plain
                // text can be advanced in one step.
                let start = i;
                while i < bytes.len() && bytes[i] != b'(' {
                    i += 1;
                }
                result.push_str(&s[start..i]);
                continue;
            }

            let close = i + 1 + bytes[i + 1..].iter().position(|&b| b == b')').unwrap();
            match map.get(&s[i + 1..close]) {
                Some(value) => result.push_str(value),
                None => result.push('?'),
            }
            i = close + 1;
        }

        result
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
