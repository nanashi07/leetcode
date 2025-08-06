// # 3479. Fruits Into Baskets III
// https://leetcode.com/problems/fruits-into-baskets-iii/

struct Solution;

impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::fruits_into_baskets_iii::Solution;

    #[test]
    fn test_num_of_unplaced_fruits_1() {
        let fruits = [4, 2, 5].to_vec();
        let baskets = [3, 5, 4].to_vec();
        assert_eq!(1, Solution::num_of_unplaced_fruits(fruits, baskets));
    }

    #[test]
    fn test_num_of_unplaced_fruits_2() {
        let fruits = [3, 6, 1].to_vec();
        let baskets = [6, 4, 7].to_vec();
        assert_eq!(0, Solution::num_of_unplaced_fruits(fruits, baskets));
    }
}
