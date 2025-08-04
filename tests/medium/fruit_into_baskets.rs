// # 904. Fruit Into Baskets
// https://leetcode.com/problems/fruit-into-baskets/description/

struct Solution;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::fruit_into_baskets::Solution;

    #[test]
    fn test_total_fruit_1() {
        let fruits = [1, 2, 1].to_vec();
        assert_eq!(3, Solution::total_fruit(fruits));
    }

    #[test]
    fn test_total_fruit_2() {
        let fruits = [0, 1, 2, 2].to_vec();
        assert_eq!(3, Solution::total_fruit(fruits));
    }

    #[test]
    fn test_total_fruit_3() {
        let fruits = [1, 2, 3, 2, 2].to_vec();
        assert_eq!(4, Solution::total_fruit(fruits));
    }
}
