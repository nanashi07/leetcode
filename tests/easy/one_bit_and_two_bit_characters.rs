// 717. 1-bit and 2-bit Characters
// https://leetcode.com/problems/1-bit-and-2-bit-characters/

struct Solution;

impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        println!("bits: {:?}", &bits);

        Self::extract(&bits[..bits.len() - 1])
    }

    fn extract(bits: &[i32]) -> bool {
        let len = bits.len();
        match len {
            0 => true,
            1 => &bits == &[0],
            _ => {
                if &bits[len - 1] == &0 && Self::extract(&bits[..len - 1]) {
                    return true;
                }
                if &bits[len - 2..] == &[1, 0] && Self::extract(&bits[..len - 2]) {
                    return true;
                }
                if &bits[len - 2..] == &[1, 1] && Self::extract(&bits[..len - 2]) {
                    return true;
                }
                false
            }
        }
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
