// 1722. Minimize Hamming Distance After Swap Operations
// https://leetcode.com/problems/minimize-hamming-distance-after-swap-operations/

struct Solution;

impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimize_hamming_distance_after_swap_operations::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_minimum_hamming_distance_1() {
        let source = [1, 2, 3, 4].to_vec();
        let target = [2, 1, 4, 5].to_vec();
        let allowed_swaps = to_vec2d([[0, 1], [2, 3]]);
        assert_eq!(
            1,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );
    }

    #[test]
    fn test_minimum_hamming_distance_2() {
        let source = [1, 2, 3, 4].to_vec();
        let target = [1, 3, 2, 4].to_vec();
        let allowed_swaps = to_vec2d([[0; 0]; 0]);
        assert_eq!(
            2,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );
    }

    #[test]
    fn test_minimum_hamming_distance_3() {
        let source = [5, 1, 2, 4, 3].to_vec();
        let target = [1, 5, 4, 2, 3].to_vec();
        let allowed_swaps = to_vec2d([[0, 4], [4, 2], [1, 3], [1, 4]]);
        assert_eq!(
            0,
            Solution::minimum_hamming_distance(source, target, allowed_swaps)
        );
    }
}
