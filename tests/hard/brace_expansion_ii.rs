// 1096. Brace Expansion II
// https://leetcode.com/problems/brace-expansion-ii/

struct Solution;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::brace_expansion_ii::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_brace_expansion_ii_1() {
        let expression = "{a,b}{c,{d,e}}".to_string();
        let output = to_string_vec(["ac", "ad", "ae", "bc", "bd", "be"]);
        assert_eq!(output, Solution::brace_expansion_ii(expression));
    }

    #[test]
    fn test_brace_expansion_ii_2() {
        let expression = "{{a,z},a{b,c},{ab,z}}".to_string();
        let output = to_string_vec(["a", "ab", "ac", "z"]);
        assert_eq!(output, Solution::brace_expansion_ii(expression));
    }
}
