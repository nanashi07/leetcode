// 1980. Find Unique Binary String
// https://leetcode.com/problems/find-unique-binary-string/

struct Solution;

impl Solution {
    pub fn find_different_binary_string(nums: Vec<String>) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_unique_binary_string::Solution;

    #[test]
    fn test_find_different_binary_string_1() {
        let nums = ["01", "10"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            "11".to_string(),
            Solution::find_different_binary_string(nums)
        );
    }

    #[test]
    fn test_find_different_binary_string_2() {
        let nums = ["00", "01"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            "11".to_string(),
            Solution::find_different_binary_string(nums)
        );
    }

    #[test]
    fn test_find_different_binary_string_3() {
        let nums = ["111", "011", "001"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(
            "101".to_string(),
            Solution::find_different_binary_string(nums)
        );
    }
}
