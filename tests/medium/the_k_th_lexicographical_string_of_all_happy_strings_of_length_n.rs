// 1415. The k-th Lexicographical String of All Happy Strings of Length n
// https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/

struct Solution;

impl Solution {
    pub fn get_happy_string(n: i32, k: i32) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::the_k_th_lexicographical_string_of_all_happy_strings_of_length_n::Solution;

    #[test]
    fn test_get_happy_string_1() {
        let n = 1;
        let k = 3;
        assert_eq!("c".to_string(), Solution::get_happy_string(n, k));
    }

    #[test]
    fn test_get_happy_string_2() {
        let n = 1;
        let k = 4;
        assert_eq!("".to_string(), Solution::get_happy_string(n, k));
    }

    #[test]
    fn test_get_happy_string_3() {
        let n = 3;
        let k = 9;
        assert_eq!("cab".to_string(), Solution::get_happy_string(n, k));
    }
}
