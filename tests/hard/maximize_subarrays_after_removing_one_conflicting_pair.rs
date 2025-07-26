// # 3480. Maximize Subarrays After Removing One Conflicting Pair
// https://leetcode.com/problems/maximize-subarrays-after-removing-one-conflicting-pair/

struct Solution;

impl Solution {
    pub fn max_subarrays(n: i32, conflicting_pairs: Vec<Vec<i32>>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::maximize_subarrays_after_removing_one_conflicting_pair::Solution;

    #[test]
    fn test_max_subarrays_1() {
        let n = 4;
        let conflicting_pairs = [[2, 3], [1, 4]]
            .iter()
            .map(|&s| s.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(9, Solution::max_subarrays(n, conflicting_pairs));
    }

    #[test]
    fn test_max_subarrays_2() {
        let n = 5;
        let conflicting_pairs = [[1, 2], [2, 5], [3, 5]]
            .iter()
            .map(|&s| s.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(12, Solution::max_subarrays(n, conflicting_pairs));
    }
}
