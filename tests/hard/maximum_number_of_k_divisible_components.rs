// 2872. Maximum Number of K-Divisible Components
// https://leetcode.com/problems/maximum-number-of-k-divisible-components/

struct Solution;

impl Solution {
    pub fn max_k_divisible_components(
        n: i32,
        edges: Vec<Vec<i32>>,
        values: Vec<i32>,
        k: i32,
    ) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximum_number_of_k_divisible_components::Solution;

    #[test]
    fn test_max_k_divisible_components_1() {
        let n = 5;
        let edges = [[0, 2], [1, 2], [1, 3], [2, 4]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let values = [1, 8, 1, 4, 4].to_vec();
        let k = 6;
        assert_eq!(2, Solution::max_k_divisible_components(n, edges, values, k));
    }

    #[test]
    fn test_max_k_divisible_components_2() {
        let n = 7;
        let edges = [[0, 1], [0, 2], [1, 3], [1, 4], [2, 5], [2, 6]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let values = [3, 0, 6, 1, 5, 2, 1].to_vec();
        let k = 3;
        assert_eq!(3, Solution::max_k_divisible_components(n, edges, values, k));
    }
}
