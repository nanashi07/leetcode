// 3666. Minimum Operations to Equalize Binary String
// https://leetcode.com/problems/minimum-operations-to-equalize-binary-string/

struct Solution;

impl Solution {
    pub fn min_operations(s: String, k: i32) -> i32 {
        let n = s.len() as i32;
        let z = s.bytes().filter(|&c| c == b'0').count() as i32;
        let o = n - z;
        if z == 0 {
            return 0;
        }
        for i in 1..=n {
            let p = i * k;
            if p % 2 != z % 2 {
                continue;
            }
            let max_p = if i % 2 == 1 { i * n - o } else { i * n - z };
            if p >= z && p <= max_p {
                return i;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_operations_to_equalize_binary_string::Solution;

    #[test]
    fn test_min_operations_1() {
        let s = "110".to_string();
        let k = 1;
        assert_eq!(1, Solution::min_operations(s, k));
    }

    #[test]
    fn test_min_operations_2() {
        let s = "0101".to_string();
        let k = 3;
        assert_eq!(2, Solution::min_operations(s, k));
    }

    #[test]
    fn test_min_operations_3() {
        let s = "101".to_string();
        let k = 2;
        assert_eq!(-1, Solution::min_operations(s, k));
    }
}
