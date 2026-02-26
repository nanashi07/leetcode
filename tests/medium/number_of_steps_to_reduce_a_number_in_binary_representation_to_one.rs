// 1404. Number of Steps to Reduce a Number in Binary Representation to One
// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/

struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let bytes = s.as_bytes();
        let len = bytes.len();
        let mut steps = 0;
        let mut carry = 0;
        for i in (1..len).rev() {
            let val = (bytes[i] - b'0') as i32 + carry;
            if val % 2 == 1 {
                steps += 2;
                carry = 1;
            } else {
                steps += 1;
                carry = val / 2;
            }
        }
        steps + carry
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_steps_to_reduce_a_number_in_binary_representation_to_one::Solution;

    #[test]
    fn test_num_steps_1() {
        let s = "1101".to_string();
        assert_eq!(6, Solution::num_steps(s));
    }

    #[test]
    fn test_num_steps_2() {
        let s = "10".to_string();
        assert_eq!(1, Solution::num_steps(s));
    }

    #[test]
    fn test_num_steps_3() {
        let s = "1".to_string();
        assert_eq!(0, Solution::num_steps(s));
    }
}
