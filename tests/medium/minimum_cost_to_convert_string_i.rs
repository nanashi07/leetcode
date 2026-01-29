// 2976. Minimum Cost to Convert String I
// https://leetcode.com/problems/minimum-cost-to-convert-string-i/

struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_cost_to_convert_string_i::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = ["a", "b", "c", "c", "e", "d"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let changed = ["b", "c", "b", "e", "b", "e"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let cost = [2, 5, 5, 1, 2, 20].to_vec();
        assert_eq!(
            28,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_2() {
        let source = "aaaa".to_string();
        let target = "bbbb".to_string();
        let original = ["a", "c"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let changed = ["c", "b"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let cost = [1, 2].to_vec();
        assert_eq!(
            12,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_3() {
        let source = "abcd".to_string();
        let target = "abce".to_string();
        let original = ["a"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let changed = ["e"]
            .iter()
            .map(|c| c.chars().next().unwrap())
            .collect::<Vec<_>>();
        let cost = [10000].to_vec();
        assert_eq!(
            -1,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }
}
