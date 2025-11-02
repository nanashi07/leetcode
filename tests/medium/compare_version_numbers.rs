// 165. Compare Version Numbers
// https://leetcode.com/problems/compare-version-numbers/

struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        println!("version!: {:?}, version2: {:?}", &version1, &version2);

        let v1 = version1
            .split(".")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let v2 = version2
            .split(".")
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let len = v1.len().max(v2.len());

        println!("v1: {:?}, v2: {:?}", &v1, &v2);

        for i in 0..len {
            let n1 = v1.get(i).unwrap_or(&0);
            let n2 = v2.get(i).unwrap_or(&0);
            if n1 > n2 {
                return 1;
            } else if n1 < n2 {
                return -1;
            }
        }

        0
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

    #[test]
    fn test_compare_version_3() {
        let version1 = "1.0".to_string();
        let version2 = "1.0.0.0".to_string();
        assert_eq!(0, Solution::compare_version(version1, version2));
    }

    #[test]
    fn test_compare_version_4() {
        let version1 = "1.0.1".to_string();
        let version2 = "1".to_string();
        assert_eq!(1, Solution::compare_version(version1, version2));
    }

    #[test]
    fn test_compare_version_5() {
        let version1 = "1".to_string();
        let version2 = "1.1".to_string();
        assert_eq!(-1, Solution::compare_version(version1, version2));
    }
}
