// # 165. Compare Version Numbers
// https://leetcode.com/problems/compare-version-numbers/

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::compare_version_numbers::Solution;

    #[test]
    fn test_compare_version_1() {
        let version1 = "1.2".to_string();
        let version2 = "1.10".to_string();
        assert_eq!(-1, Solution::compare_version(version1, version2));
    }

    #[test]
    fn test_compare_version_2() {
        let version1 = "1.01".to_string();
        let version2 = "1.001".to_string();
        assert_eq!(0, Solution::compare_version(version1, version2));
    }
}
