// 1545. Find Kth Bit in Nth Binary String
// https://leetcode.com/problems/find-kth-bit-in-nth-binary-string/

struct Solution;

impl Solution {
    pub fn find_kth_bit(n: i32, k: i32) -> char {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_kth_bit_in_nth_binary_string::Solution;

    #[test]
    fn test_find_kth_bit_1() {
        let n = 3;
        let k = 1;
        assert_eq!('0', Solution::find_kth_bit(n, k));
    }

    #[test]
    fn test_find_kth_bit_2() {
        let n = 4;
        let k = 11;
        assert_eq!('1', Solution::find_kth_bit(n, k));
    }
}
