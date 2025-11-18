// 717. 1-bit and 2-bit Characters
// https://leetcode.com/problems/1-bit-and-2-bit-characters/

struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::one_bit_and_two_bit_characters::Solution;

    #[test]
    fn test_is_one_bit_character_1() {
        let bits = [1, 0, 0].to_vec();
        assert_eq!(true, Solution::is_one_bit_character(bits));
    }

    #[test]
    fn test_is_one_bit_character_2() {
        let bits = [1, 1, 1, 0].to_vec();
        assert_eq!(false, Solution::is_one_bit_character(bits));
    }
}
