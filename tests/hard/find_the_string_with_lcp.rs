// 2573. Find the String with LCP
// https://leetcode.com/problems/find-the-string-with-lcp/

struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_string_with_lcp::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_find_the_string_1() {
        let lcp = to_vec2d([[4, 0, 2, 0], [0, 3, 0, 1], [2, 0, 2, 0], [0, 1, 0, 1]]);
        let output = "abab".to_string();
        assert_eq!(output, Solution::find_the_string(lcp));
    }

    #[test]
    fn test_find_the_string_2() {
        let lcp = to_vec2d([[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 1]]);
        let output = "aaaa".to_string();
        assert_eq!(output, Solution::find_the_string(lcp));
    }

    #[test]
    fn test_find_the_string_3() {
        let lcp = to_vec2d([[4, 3, 2, 1], [3, 3, 2, 1], [2, 2, 2, 1], [1, 1, 1, 3]]);
        let output = "".to_string();
        assert_eq!(output, Solution::find_the_string(lcp));
    }
}
