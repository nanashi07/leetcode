// 3130. Find All Possible Stable Binary Arrays II
// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-ii/

struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_all_possible_stable_binary_arrays_ii::Solution;

    #[test]
    fn test_number_of_stable_arrays_1() {
        let zero = 1;
        let one = 1;
        let limit = 2;
        assert_eq!(2, Solution::number_of_stable_arrays(zero, one, limit));
    }

    #[test]
    fn test_number_of_stable_arrays_2() {
        let zero = 1;
        let one = 2;
        let limit = 1;
        assert_eq!(1, Solution::number_of_stable_arrays(zero, one, limit));
    }

    #[test]
    fn test_number_of_stable_arrays_3() {
        let zero = 3;
        let one = 3;
        let limit = 2;
        assert_eq!(14, Solution::number_of_stable_arrays(zero, one, limit));
    }
}
