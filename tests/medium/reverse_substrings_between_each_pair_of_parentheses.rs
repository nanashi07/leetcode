// 1190. Reverse Substrings Between Each Pair of Parentheses
// https://leetcode.com/problems/reverse-substrings-between-each-pair-of-parentheses/

struct Solution;

impl Solution {
    pub fn reverse_parentheses(s: String) -> String {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len() as isize;

        // Index of the bracket each parenthesis pairs with, -1 for plain letters.
        let mut mate = vec![-1isize; chars.len()];
        let mut opens: Vec<isize> = Vec::new();
        for i in 0..n {
            match chars[i as usize] {
                '(' => opens.push(i),
                ')' => {
                    if let Some(j) = opens.pop() {
                        mate[i as usize] = j;
                        mate[j as usize] = i;
                    }
                }
                _ => {}
            }
        }

        // Walk left to right, flipping the step sign at every pair: each letter is emitted once.
        let mut out = String::with_capacity(chars.len());
        let (mut i, mut step) = (0isize, 1isize);
        while i >= 0 && i < n {
            match mate[i as usize] {
                -1 => out.push(chars[i as usize]),
                j => {
                    i = j;
                    step = -step;
                }
            }
            i += step;
        }

        out
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::reverse_substrings_between_each_pair_of_parentheses::Solution;

    #[test]
    fn test_reverse_parentheses_1() {
        let s = "(abcd)".to_string();
        assert_eq!("dcba".to_string(), Solution::reverse_parentheses(s));
    }

    #[test]
    fn test_reverse_parentheses_2() {
        let s = "(u(love)i)".to_string();
        assert_eq!("iloveu".to_string(), Solution::reverse_parentheses(s));
    }

    #[test]
    fn test_reverse_parentheses_3() {
        let s = "(ed(et(oc))el)".to_string();
        assert_eq!("leetcode".to_string(), Solution::reverse_parentheses(s));
    }
}
