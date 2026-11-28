// 3947. Maximum Number of Items From Sale II
// https://leetcode.com/problems/maximum-number-of-items-from-sale-ii/

struct Solution;

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_number_of_items_from_sale_ii::Solution;

    #[test]
    fn test_maximum_sale_items_1() {
        let items: Vec<Vec<i32>> = vec![vec![1, 6], vec![2, 4], vec![3, 5]];
        let budget: i32 = 19;
        assert_eq!(5, Solution::maximum_sale_items(items, budget));
    }

    #[test]
    fn test_maximum_sale_items_2() {
        let items: Vec<Vec<i32>> = vec![
            vec![2, 8],
            vec![1, 10],
            vec![6, 6],
            vec![4, 12],
            vec![5, 20],
            vec![5, 17],
        ];
        let budget: i32 = 35;
        assert_eq!(7, Solution::maximum_sale_items(items, budget));
    }
}
