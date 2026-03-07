// 1888. Minimum Number of Flips to Make the Binary String Alternating
// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/

struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_flips_to_make_the_binary_string_alternating::Solution;

    #[test]
    fn test_min_flips_1() {
        let s = "111000".to_string();
        assert_eq!(2, Solution::min_flips(s));
    }

    #[test]
    fn test_min_flips_2() {
        let s = "010".to_string();
        assert_eq!(0, Solution::min_flips(s));
    }

    #[test]
    fn test_min_flips_3() {
        let s = "1110".to_string();
        assert_eq!(1, Solution::min_flips(s));
    }
}
