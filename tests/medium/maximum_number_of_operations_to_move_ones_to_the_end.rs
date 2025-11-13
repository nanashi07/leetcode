// 3228. Maximum Number of Operations to Move Ones to the End
// https://leetcode.com/problems/maximum-number-of-operations-to-move-ones-to-the-end/

struct Solution;

impl Solution {
    pub fn max_operations(s: String) -> i32 {
        println!("s: {:?}", &s);

        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        let mut operations = 0;
        let mut ones_count = 0;

        for i in 0..n {
            if chars[i] == '1' {
                ones_count += 1;
            } else {
                // We found a '0'
                // If there's a '1' before this '0', all those '1's need to move past it
                // Only count if the previous character was '1' (to avoid counting consecutive 0s multiple times)
                if i > 0 && chars[i - 1] == '1' {
                    operations += ones_count;
                }
            }
        }

        operations
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_operations_to_move_ones_to_the_end::Solution;

    #[test]
    fn test_max_operations_1() {
        let s = "1001101".to_string();
        assert_eq!(4, Solution::max_operations(s));
    }

    #[test]
    fn test_max_operations_2() {
        let s = "00111".to_string();
        assert_eq!(0, Solution::max_operations(s));
    }
}
