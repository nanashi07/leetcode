// 3614. Process String with Special Operations II
// https://leetcode.com/problems/process-string-with-special-operations-ii/

struct Solution;

impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::process_string_with_special_operations_ii::Solution;

    #[test]
    fn test_process_str_1() {
        let s = "a#b%*".to_string();
        let k = 1;
        assert_eq!('a', Solution::process_str(s, k));
    }

    #[test]
    fn test_process_str_2() {
        let s = "cd%#*#".to_string();
        let k = 3;
        assert_eq!('d', Solution::process_str(s, k));
    }

    #[test]
    fn test_process_str_3() {
        let s = "z*#".to_string();
        let k = 0;
        assert_eq!('.', Solution::process_str(s, k));
    }
}
