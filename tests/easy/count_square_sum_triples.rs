// 1925. Count Square Sum Triples
// https://leetcode.com/problems/count-square-sum-triples/

struct Solution;

impl Solution {
    pub fn count_triples(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_square_sum_triples::Solution;

    #[test]
    fn test_count_triples_1() {
        let n = 5;
        assert_eq!(2, Solution::count_triples(n));
    }

    #[test]
    fn test_count_triples_2() {
        let n = 10;
        assert_eq!(4, Solution::count_triples(n));
    }
}
