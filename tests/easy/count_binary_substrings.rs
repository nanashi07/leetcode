// 696. Count Binary Substrings
// https://leetcode.com/problems/count-binary-substrings/

struct Solution;

impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        println!("s: {s}");

        let mut p = &s[0..1];
        let mut c = 0;
        for i in 1..s.len() {
            if p != &s[i..=i] {
                // find pair
                c += 1;
                for l in 0..(i.min(s.len() - 1 - i)) {
                    if i >= 2 + l
                        && i + 1 + l < s.len()
                        && &s[i + l..=i + l] == &s[i + 1 + l..=i + 1 + l]
                        && &s[i - 2 - l..=i - 2 - l] == &s[i - 1 - l..=i - 1 - l]
                    {
                        c += 1;
                    } else {
                        break;
                    }
                }
            }
            p = &s[i..=i];
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_binary_substrings::Solution;

    #[test]
    fn test_count_binary_substrings_1() {
        let s = "00110011".to_string();
        assert_eq!(6, Solution::count_binary_substrings(s));
    }

    #[test]
    fn test_count_binary_substrings_2() {
        let s = "10101".to_string();
        assert_eq!(4, Solution::count_binary_substrings(s));
    }
}
