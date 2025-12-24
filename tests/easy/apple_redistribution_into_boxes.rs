// 3074. Apple Redistribution into Boxes
// https://leetcode.com/problems/apple-redistribution-into-boxes/

struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, capacity: Vec<i32>) -> i32 {
        println!("apple: {:?}, capacity: {:?}", &apple, &capacity);

        let mut sum_of_apple = apple.iter().sum::<i32>();
        let mut capacity = capacity;
        capacity.sort_unstable();

        for i in (0..capacity.len()).rev() {
            sum_of_apple -= capacity[i];
            if sum_of_apple <= 0 {
                return (capacity.len() - i) as i32;
            }
        }

        capacity.len() as i32
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
