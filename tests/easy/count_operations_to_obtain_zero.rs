// 2169. Count Operations to Obtain Zero
// https://leetcode.com/problems/count-operations-to-obtain-zero/

struct Solution;

impl Solution {
    pub fn count_operations(num1: i32, num2: i32) -> i32 {
        println!("num1: {num1}, num2: {num2}");

        let mut c = 0;
        let mut num1 = num1;
        let mut num2 = num2;

        while num1 > 0 && num2 > 0 {
            if num1 > num2 {
                num1 -= num2;
                c += 1;
            } else {
                num2 -= num1;
                c += 1;
            }
        }

        c
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_operations_to_obtain_zero::Solution;

    #[test]
    fn test_count_operations_1() {
        let num1 = 2;
        let num2 = 3;
        assert_eq!(3, Solution::count_operations(num1, num2));
    }

    #[test]
    fn test_count_operations_2() {
        let num1 = 10;
        let num2 = 10;
        assert_eq!(1, Solution::count_operations(num1, num2));
    }
}
