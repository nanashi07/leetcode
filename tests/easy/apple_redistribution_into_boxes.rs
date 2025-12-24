// 3074. Apple Redistribution into Boxes
// https://leetcode.com/problems/apple-redistribution-into-boxes/

struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::apple_redistribution_into_boxes::Solution;

    #[test]
    fn test_minimum_boxes_1() {
        let apple = [1, 3, 2].to_vec();
        let capacity = [4, 3, 1, 5, 2].to_vec();
        assert_eq!(2, Solution::minimum_boxes(apple, capacity));
    }

    #[test]
    fn test_minimum_boxes_2() {
        let apple = [5, 5, 5].to_vec();
        let capacity = [2, 4, 2, 7].to_vec();
        assert_eq!(4, Solution::minimum_boxes(apple, capacity));
    }
}
