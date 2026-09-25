// 1096. Brace Expansion II
// https://leetcode.com/problems/brace-expansion-ii/

use std::collections::BTreeSet;

struct Solution;

impl Solution {
    pub fn brace_expansion_ii(expression: String) -> Vec<String> {
        let mut parser = Parser {
            expression: expression.as_bytes(),
            position: 0,
        };
        parser.parse_union().into_iter().collect()
    }
}

struct Parser<'a> {
    expression: &'a [u8],
    position: usize,
}

impl Parser<'_> {
    fn parse_union(&mut self) -> BTreeSet<String> {
        let mut result = BTreeSet::new();
        result.extend(self.parse_concatenation());

        while self.position < self.expression.len() && self.expression[self.position] == b',' {
            self.position += 1;
            result.extend(self.parse_concatenation());
        }

        result
    }

    fn parse_concatenation(&mut self) -> BTreeSet<String> {
        let mut result = BTreeSet::from([String::new()]);

        while self.position < self.expression.len()
            && self.expression[self.position] != b','
            && self.expression[self.position] != b'}'
        {
            let atom = if self.expression[self.position] == b'{' {
                self.position += 1;
                let nested = self.parse_union();
                self.position += 1;
                nested
            } else {
                let character = self.expression[self.position] as char;
                self.position += 1;
                BTreeSet::from([character.to_string()])
            };

            let mut concatenated = BTreeSet::new();
            for prefix in &result {
                for suffix in &atom {
                    concatenated.insert(format!("{prefix}{suffix}"));
                }
            }
            result = concatenated;
        }

        result
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
