// # 2300. Successful Pairs of Spells and Potions
// https://leetcode.com/problems/successful-pairs-of-spells-and-potions/

struct Solution;

impl Solution {
    pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::successful_pairs_of_spells_and_potions::Solution;

    #[test]
    fn test_successful_pairs_1() {
        let spells = [5, 1, 3].to_vec();
        let potions = [1, 2, 3, 4, 5].to_vec();
        let success = 7;
        assert_eq!(
            [4, 0, 3].to_vec(),
            Solution::successful_pairs(spells, potions, success)
        );
    }

    #[test]
    fn test_successful_pairs_2() {
        let spells = [3, 1, 2].to_vec();
        let potions = [8, 5, 8].to_vec();
        let success = 16;
        assert_eq!(
            [2, 0, 2].to_vec(),
            Solution::successful_pairs(spells, potions, success)
        );
    }
}
