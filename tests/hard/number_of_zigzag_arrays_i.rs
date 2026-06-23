// 3699. Number of ZigZag Arrays I
// https://leetcode.com/problems/number-of-zigzag-arrays-i/

struct Solution;

impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::number_of_zigzag_arrays_i::Solution;

    #[test]
    fn test_zig_zag_arrays_1() {
        let n = 3;
        let l = 4;
        let r = 5;
        assert_eq!(2, Solution::zig_zag_arrays(n, l, r));
    }

    #[test]
    fn test_zig_zag_arrays_2() {
        let n = 3;
        let l = 1;
        let r = 3;
        assert_eq!(10, Solution::zig_zag_arrays(n, l, r));
    }
}
