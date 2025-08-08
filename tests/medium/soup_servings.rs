// # 808. Soup Servings
// https://leetcode.com/problems/soup-servings/description/?envType=daily-question&envId=2025-08-08

struct Solution;

impl Solution {
    pub fn soup_servings(n: i32) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::soup_servings::Solution;

    #[test]
    fn test_soup_servings_1() {
        let n = 50;
        assert_eq!(0.62500, Solution::soup_servings(n));
    }

    #[test]
    fn test_soup_servings_2() {
        let n = 100;
        assert_eq!(0.71875, Solution::soup_servings(n));
    }
}
