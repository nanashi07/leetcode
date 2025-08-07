// # 3363. Find the Maximum Number of Fruits Collected
// https://leetcode.com/problems/find-the-maximum-number-of-fruits-collected/

struct Solution;

impl Solution {
    pub fn max_collected_fruits(fruits: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_maximum_number_of_fruits_collected::Solution;

    #[test]
    fn test_max_collected_fruits_1() {
        let fruits = [
            [1, 2, 3, 4],
            [5, 6, 8, 7],
            [9, 10, 11, 12],
            [13, 14, 15, 16],
        ]
        .iter()
        .map(|&l| l.to_vec())
        .collect::<Vec<Vec<i32>>>();
        assert_eq!(100, Solution::max_collected_fruits(fruits));
    }

    #[test]
    fn test_max_collected_fruits_2() {
        let fruits = [[1, 1], [1, 1]]
            .iter()
            .map(|&l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(4, Solution::max_collected_fruits(fruits));
    }
}
