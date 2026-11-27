// 3946. Maximum Number of Items From Sale I
// https://leetcode.com/problems/maximum-number-of-items-from-sale-i/

struct Solution;

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_items_from_sale_i::Solution;

    #[test]
    fn test_maximum_sale_items_1() {
        let items: Vec<Vec<i32>> = vec![vec![6, 2], vec![2, 6], vec![3, 4]];
        let budget: i32 = 9;
        assert_eq!(4, Solution::maximum_sale_items(items, budget));
    }

    #[test]
    fn test_maximum_sale_items_2() {
        let items: Vec<Vec<i32>> =
            vec![vec![2, 4], vec![3, 2], vec![4, 1], vec![6, 4], vec![12, 4]];
        let budget: i32 = 8;
        assert_eq!(4, Solution::maximum_sale_items(items, budget));
    }
}
