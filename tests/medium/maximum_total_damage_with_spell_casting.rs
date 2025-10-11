// # 3186. Maximum Total Damage With Spell Casting
// https://leetcode.com/problems/maximum-total-damage-with-spell-casting/

struct Solution;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_total_damage_with_spell_casting::Solution;

    #[test]
    fn test_maximum_total_damage_1() {
        let power = [1, 1, 3, 4].to_vec();
        assert_eq!(6, Solution::maximum_total_damage(power));
    }

    #[test]
    fn test_maximum_total_damage_2() {
        let power = [7, 1, 6, 6].to_vec();
        assert_eq!(13, Solution::maximum_total_damage(power));
    }
}
