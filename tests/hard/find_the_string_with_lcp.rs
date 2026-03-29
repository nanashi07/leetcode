// 2573. Find the String with LCP
// https://leetcode.com/problems/find-the-string-with-lcp/

struct Solution;

impl Solution {
    pub fn find_the_string(lcp: Vec<Vec<i32>>) -> String {
        let n = lcp.len();
        let mut chars = vec![0u8; n]; // 0 = unassigned, 1..=26 maps to 'a'..'z'
        let mut next_char = 1u8;

        // Greedy assignment: lcp[i][j] > 0 implies s[i] == s[j]
        for i in 0..n {
            for j in 0..i {
                if lcp[i][j] > 0 {
                    if chars[i] == 0 {
                        chars[i] = chars[j];
                    } else if chars[i] != chars[j] {
                        return String::new();
                    }
                }
            }
            if chars[i] == 0 {
                if next_char > 26 {
                    return String::new();
                }
                chars[i] = next_char;
                next_char += 1;
            }
        }

        // Validate: compute the true LCP from the constructed string (O(n²) DP)
        // computed[i][j] = lcp of suffix starting at i and j
        let mut computed = vec![vec![0i32; n]; n];
        for i in (0..n).rev() {
            for j in (0..n).rev() {
                if chars[i] == chars[j] {
                    computed[i][j] = if i + 1 < n && j + 1 < n {
                        computed[i + 1][j + 1] + 1
                    } else {
                        1
                    };
                }
            }
        }

        if computed != lcp {
            return String::new();
        }

        chars.iter().map(|&c| (b'a' + c - 1) as char).collect()
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
