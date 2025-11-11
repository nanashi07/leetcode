// 474. Ones and Zeroes
// https://leetcode.com/problems/ones-and-zeroes/

struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::ones_and_zeroes::Solution;

    #[test]
    fn test_find_max_form_1() {
        let strs = ["10", "0001", "111001", "1", "0"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let m = 5;
        let n = 3;
        assert_eq!(4, Solution::find_max_form(strs, m, n));
    }

    #[test]
    fn test_find_max_form_2() {
        let strs = ["10", "0", "1"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let m = 1;
        let n = 1;
        assert_eq!(2, Solution::find_max_form(strs, m, n));
    }
}
