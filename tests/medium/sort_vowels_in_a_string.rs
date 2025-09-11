// # 2785. Sort Vowels in a String
// https://leetcode.com/problems/sort-vowels-in-a-string/

struct Solution;

impl Solution {
    pub fn sort_vowels(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::sort_vowels_in_a_string::Solution;

    #[test]
    fn test_sort_vowels_1() {
        let s = "lEetcOde".to_owned();
        assert_eq!("lEOtcede".to_owned(), Solution::sort_vowels(s));
    }

    #[test]
    fn test_sort_vowels_2() {
        let s = "lYmpH".to_owned();
        assert_eq!("lYmpH".to_owned(), Solution::sort_vowels(s));
    }
}
