// 2977. Minimum Cost to Convert String II
// https://leetcode.com/problems/minimum-cost-to-convert-string-ii/

struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<String>,
        changed: Vec<String>,
        cost: Vec<i32>,
    ) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::minimum_cost_to_convert_string_ii::Solution;

    #[test]
    fn test_minimum_cost_1() {
        let source = "abcd".to_string();
        let target = "acbe".to_string();
        let original = ["a", "b", "c", "c", "e", "d"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["b", "c", "b", "e", "b", "e"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [2, 5, 5, 1, 2, 20].to_vec();
        assert_eq!(
            28,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_2() {
        let source = "abcdefgh".to_string();
        let target = "acdeeghh".to_string();
        let original = ["bcd", "fgh", "thh"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["cde", "thh", "ghh"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [1, 3, 5].to_vec();
        assert_eq!(
            9,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }

    #[test]
    fn test_minimum_cost_3() {
        let source = "abcdefgh".to_string();
        let target = "addddddd".to_string();
        let original = ["bcd", "defgh"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let changed = ["ddd", "ddddd"]
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>();
        let cost = [100, 1578].to_vec();
        assert_eq!(
            -1,
            Solution::minimum_cost(source, target, original, changed, cost)
        );
    }
}
