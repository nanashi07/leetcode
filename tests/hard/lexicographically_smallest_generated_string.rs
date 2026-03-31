// 3474. Lexicographically Smallest Generated String
// https://leetcode.com/problems/lexicographically-smallest-generated-string/

struct Solution;

impl Solution {
    pub fn generate_string(str1: String, str2: String) -> String {
        let n1 = str1.len();
        let n2 = str2.len();
        let n = n1 + n2 - 1;

        let s1: Vec<u8> = str1.bytes().collect();
        let s2: Vec<u8> = str2.bytes().collect();

        let mut result: Vec<u8> = vec![b'a'; n];
        let mut constrained = vec![false; n];

        // Apply T constraints: force result[i..i+n2] == str2
        for i in 0..n1 {
            if s1[i] == b'T' {
                for j in 0..n2 {
                    let pos = i + j;
                    if constrained[pos] && result[pos] != s2[j] {
                        return String::new(); // Conflict between two T windows
                    }
                    result[pos] = s2[j];
                    constrained[pos] = true;
                }
            }
        }

        // Fix F constraints left-to-right: result[i..i+n2] must != str2
        for i in 0..n1 {
            if s1[i] == b'F' {
                // Check if current window equals str2
                let window_matches = (0..n2).all(|j| result[i + j] == s2[j]);
                if window_matches {
                    // Find the rightmost unconstrained position in [i, i+n2)
                    // (unconstrained positions have str2[j-i]='a', so fix is always 'b')
                    match (0..n2).rev().find(|&j| !constrained[i + j]) {
                        None => return String::new(), // All constrained, impossible to fix
                        Some(j) => result[i + j] = b'b',
                    }
                }
            }
        }

        String::from_utf8(result).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::lexicographically_smallest_generated_string::Solution;

    #[test]
    fn test_generate_string_1() {
        let str1 = "TFTF".to_string();
        let str2 = "ab".to_string();
        assert_eq!("ababa".to_string(), Solution::generate_string(str1, str2));
    }

    #[test]
    fn test_generate_string_2() {
        let str1 = "TFTF".to_string();
        let str2 = "abc".to_string();
        assert_eq!("".to_string(), Solution::generate_string(str1, str2));
    }

    #[test]
    fn test_generate_string_3() {
        let str1 = "F".to_string();
        let str2 = "d".to_string();
        assert_eq!("a".to_string(), Solution::generate_string(str1, str2));
    }
}
