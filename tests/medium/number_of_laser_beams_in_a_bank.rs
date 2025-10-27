// # 2125. Number of Laser Beams in a Bank
// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/

struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_laser_beams_in_a_bank::Solution;

    #[test]
    fn test_number_of_beams_1() {
        let bank = ["011001", "000000", "010100", "001000"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(8, Solution::number_of_beams(bank));
    }

    #[test]
    fn test_number_of_beams_2() {
        let bank = ["000", "111", "000"]
            .iter()
            .map(|&s| s.to_string())
            .collect::<Vec<_>>();
        assert_eq!(0, Solution::number_of_beams(bank));
    }
}
