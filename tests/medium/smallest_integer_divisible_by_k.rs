// 1015. Smallest Integer Divisible by K
// https://leetcode.com/problems/smallest-integer-divisible-by-k/

struct Solution;

impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::smallest_integer_divisible_by_k::Solution;

    #[test]
    fn test_smallest_repunit_div_by_k_1() {
        let k = 1;
        assert_eq!(1, Solution::smallest_repunit_div_by_k(k));
    }

    #[test]
    fn test_smallest_repunit_div_by_k_2() {
        let k = 2;
        assert_eq!(-1, Solution::smallest_repunit_div_by_k(k));
    }

    #[test]
    fn test_smallest_repunit_div_by_k_3() {
        let k = 3;
        assert_eq!(3, Solution::smallest_repunit_div_by_k(k));
    }
}
