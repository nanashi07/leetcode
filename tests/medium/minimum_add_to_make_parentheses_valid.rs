// 921. Minimum Add to Make Parentheses Valid
// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/

struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        // `open` counts '(' still waiting for a partner; `missing` counts ')' that arrived with
        // nothing to close. Every unmatched bracket needs exactly one insertion, so the answer is
        // the two leftovers combined.
        let (mut open, mut missing) = (0i32, 0i32);

        for bracket in s.bytes() {
            if bracket == b'(' {
                open += 1;
            } else if open > 0 {
                open -= 1;
            } else {
                missing += 1;
            }
        }

        open + missing
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_add_to_make_parentheses_valid::Solution;

    #[test]
    fn test_min_add_to_make_valid_1() {
        let s = "())".to_string();
        assert_eq!(1, Solution::min_add_to_make_valid(s));
    }

    #[test]
    fn test_min_add_to_make_valid_2() {
        let s = "(((".to_string();
        assert_eq!(3, Solution::min_add_to_make_valid(s));
    }
}
