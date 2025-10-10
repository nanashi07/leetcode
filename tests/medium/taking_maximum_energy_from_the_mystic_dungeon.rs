// # 3147. Taking Maximum Energy From the Mystic Dungeon
// https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon/

struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::taking_maximum_energy_from_the_mystic_dungeon::Solution;

    #[test]
    fn test_maximum_energy_1() {
        let energy = [5, 2, -10, -5, 1].to_vec();
        let k = 3;
        assert_eq!(3, Solution::maximum_energy(energy, k));
    }

    #[test]
    fn test_maximum_energy_2() {
        let energy = [-2, -3, -1].to_vec();
        let k = 2;
        assert_eq!(-1, Solution::maximum_energy(energy, k));
    }
}
