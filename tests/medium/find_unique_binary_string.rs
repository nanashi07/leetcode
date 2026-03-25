// 1980. Find Unique Binary String
// https://leetcode.com/problems/find-unique-binary-string/

struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        let n = nums[0].len();
        let mut refs: Vec<&str> = nums.iter().map(|s| s.as_str()).collect();
        let mut result = String::with_capacity(n);

        for depth in 0..n {
            let (zeros, ones): (Vec<&str>, Vec<&str>) =
                refs.iter().partition(|s| s.as_bytes()[depth] == b'0');
            if zeros.len() < ones.len() {
                result.push('0');
                refs = zeros;
            } else {
                result.push('1');
                refs = ones;
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_unique_binary_string::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_find_different_binary_string_1() {
        let nums = to_string_vec(["01", "10"]);
        assert_eq!(
            "11".to_string(),
            Solution::find_different_binary_string(nums)
        );
    }

    #[test]
    fn test_find_different_binary_string_2() {
        let nums = to_string_vec(["00", "01"]);
        assert_eq!(
            "11".to_string(),
            Solution::find_different_binary_string(nums)
        );
    }

    #[test]
    fn test_find_different_binary_string_3() {
        let nums = to_string_vec(["111", "011", "001"]);
        assert_eq!(
            "101".to_string(),
            Solution::find_different_binary_string(nums)
        );
    }
}
