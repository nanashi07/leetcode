// 868. Binary Gap
// https://leetcode.com/problems/binary-gap/

struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::binary_gap::Solution;

    #[test]
    fn test_binary_gap_1() {
        let n = 22;
        assert_eq!(2, Solution::binary_gap(n));
    }

    #[test]
    fn test_binary_gap_2() {
        let n = 8;
        assert_eq!(0, Solution::binary_gap(n));
    }

    #[test]
    fn test_binary_gap_3() {
        let n = 5;
        assert_eq!(2, Solution::binary_gap(n));
    }
}
