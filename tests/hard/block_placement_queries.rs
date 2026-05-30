// 3161. Block Placement Queries
// https://leetcode.com/problems/block-placement-queries/

struct Solution;

impl Solution {
    pub fn get_results(queries: Vec<Vec<i32>>) -> Vec<bool> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::block_placement_queries::Solution;

    #[test]
    fn test_get_results_1() {
        let queries = [
            [1, 2].to_vec(),
            [2, 3, 3].to_vec(),
            [2, 3, 1].to_vec(),
            [2, 2, 2].to_vec(),
        ]
        .to_vec();
        assert_eq!([false, true, true].to_vec(), Solution::get_results(queries));
    }

    #[test]
    fn test_get_results_2() {
        let queries = [
            [1, 7].to_vec(),
            [2, 7, 6].to_vec(),
            [1, 2].to_vec(),
            [2, 7, 5].to_vec(),
            [2, 7, 6].to_vec(),
        ]
        .to_vec();
        assert_eq!([false, true, true].to_vec(), Solution::get_results(queries));
    }
}
