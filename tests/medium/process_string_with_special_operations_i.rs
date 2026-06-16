// 3612. Process String with Special Operations I
// https://leetcode.com/problems/process-string-with-special-operations-i/

struct Solution;

impl Solution {
    pub fn process_str(s: String) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::process_string_with_special_operations_i::Solution;

    #[test]
    fn test_process_str_1() {
        let s = "a#b%*".to_string();
        assert_eq!("ba".to_string(), Solution::process_str(s));
    }

    #[test]
    fn test_process_str_2() {
        let s = "z*#".to_string();
        assert_eq!("".to_string(), Solution::process_str(s));
    }
}
