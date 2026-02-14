// 799. Champagne Tower
// https://leetcode.com/problems/champagne-tower/

struct Solution;

impl Solution {
    pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::champagne_tower::Solution;

    #[test]
    fn test_champagne_tower_1() {
        let oured = 1;
        let query_row = 1;
        let query_glass = 1;
        assert_eq!(
            0.00000,
            Solution::champagne_tower(oured, query_row, query_glass)
        );
    }

    #[test]
    fn test_champagne_tower_2() {
        let oured = 2;
        let query_row = 1;
        let query_glass = 1;
        assert_eq!(
            0.50000,
            Solution::champagne_tower(oured, query_row, query_glass)
        );
    }

    #[test]
    fn test_champagne_tower_3() {
        let oured = 100000009;
        let query_row = 33;
        let query_glass = 17;
        assert_eq!(
            1.00000,
            Solution::champagne_tower(oured, query_row, query_glass)
        );
    }
}
