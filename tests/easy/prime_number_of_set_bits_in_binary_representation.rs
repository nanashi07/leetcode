// 762. Prime Number of Set Bits in Binary Representation
// https://leetcode.com/problems/prime-number-of-set-bits-in-binary-representation/

struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        println!("left: {left}, right: {right}");

        let mut count = 0;
        for i in left..=right {
            let mut n = i;
            let mut bits = 0;
            while n > 0 {
                if n % 2 == 1 {
                    bits += 1;
                }
                n = n >> 1;
            }
            match bits {
                2 | 3 | 5 | 7 | 11 | 13 | 17 | 19 => count += 1,
                _ => {}
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::prime_number_of_set_bits_in_binary_representation::Solution;

    #[test]
    fn test_count_prime_set_bits_1() {
        let left = 6;
        let right = 10;
        assert_eq!(4, Solution::count_prime_set_bits(left, right));
    }

    #[test]
    fn test_count_prime_set_bits_2() {
        let left = 10;
        let right = 15;
        assert_eq!(5, Solution::count_prime_set_bits(left, right));
    }
}
