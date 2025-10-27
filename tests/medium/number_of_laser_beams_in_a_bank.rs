// # 2125. Number of Laser Beams in a Bank
// https://leetcode.com/problems/number-of-laser-beams-in-a-bank/

struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut total_beams = 0;
        let mut prev_count = 0;

        for row in bank {
            // Count the number of devices (1's) in this row
            let curr_count = row.chars().filter(|&c| c == '1').count() as i32;

            // If current row has devices
            if curr_count > 0 {
                // Add beams between previous row and current row
                total_beams += prev_count * curr_count;
                // Update previous count for next iteration
                prev_count = curr_count;
            }
        }

        total_beams
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
