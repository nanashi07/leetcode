// # 3494. Find the Minimum Amount of Time to Brew Potions
// https://leetcode.com/problems/find-the-minimum-amount-of-time-to-brew-potions/

struct Solution;

impl Solution {
    pub fn min_time(skill: Vec<i32>, mana: Vec<i32>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_minimum_amount_of_time_to_brew_potions::Solution;

    #[test]
    fn test_min_time_1() {
        let skill = [1, 5, 2, 4].to_vec();
        let mana = [5, 1, 4, 2].to_vec();
        assert_eq!(110, Solution::min_time(skill, mana));
    }

    #[test]
    fn test_min_time_2() {
        let skill = [1, 1, 1].to_vec();
        let mana = [1, 1, 1].to_vec();
        assert_eq!(5, Solution::min_time(skill, mana));
    }

    #[test]
    fn test_min_time_3() {
        let skill = [1, 2, 3, 4].to_vec();
        let mana = [1, 2].to_vec();
        assert_eq!(21, Solution::min_time(skill, mana));
    }
}
