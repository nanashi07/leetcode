// 2011. Final Value of Variable After Performing Operations
// https://leetcode.com/problems/final-value-of-variable-after-performing-operations/

struct Solution;

impl Solution {
    pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
        println!("operations: {:?}", &operations);

        let mut x = 0;

        for operation in operations {
            match operation.as_str() {
                "++X" => x += 1,
                "X++" => x += 1,
                "--X" => x -= 1,
                "X--" => x -= 1,
                _ => {}
            }
        }

        x
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::final_value_of_variable_after_performing_operations::Solution;
    use crate::shared::vec2d::to_string_vec;

    #[test]
    fn test_final_value_after_operations_1() {
        let operations = to_string_vec(["--X", "X++", "X++"]);
        assert_eq!(1, Solution::final_value_after_operations(operations));
    }

    #[test]
    fn test_final_value_after_operations_2() {
        let operations = to_string_vec(["++X", "++X", "X++"]);
        assert_eq!(3, Solution::final_value_after_operations(operations));
    }

    #[test]
    fn test_final_value_after_operations_3() {
        let operations = to_string_vec(["X++", "++X", "--X", "X--"]);
        assert_eq!(0, Solution::final_value_after_operations(operations));
    }
}
