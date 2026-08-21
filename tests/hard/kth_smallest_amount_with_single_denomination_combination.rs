// 3116. Kth Smallest Amount With Single Denomination Combination
// https://leetcode.com/problems/kth-smallest-amount-with-single-denomination-combination/

struct Solution;

impl Solution {
    pub fn find_kth_smallest(coins: Vec<i32>, k: i32) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::kth_smallest_amount_with_single_denomination_combination::Solution;

    #[test]
    fn test_find_kth_smallest_1() {
        let coins = [3, 6, 9].to_vec();
        let k = 3;
        assert_eq!(9, Solution::find_kth_smallest(coins, k));
    }

    #[test]
    fn test_find_kth_smallest_2() {
        let coins = [5, 2].to_vec();
        let k = 7;
        assert_eq!(12, Solution::find_kth_smallest(coins, k));
    }
}
