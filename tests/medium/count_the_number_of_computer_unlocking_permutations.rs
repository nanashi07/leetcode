// 3577. Count the Number of Computer Unlocking Permutations
// https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations/

struct Solution;

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_the_number_of_computer_unlocking_permutations::Solution;

    #[test]
    fn test_count_permutations_1() {
        let complexity = [1, 2, 3].to_vec();
        assert_eq!(2, Solution::count_permutations(complexity));
    }

    #[test]
    fn test_count_permutations_2() {
        let complexity = [3, 3, 3, 4, 4, 4].to_vec();
        assert_eq!(0, Solution::count_permutations(complexity));
    }
}
