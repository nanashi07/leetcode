// 2211. Count Collisions on a Road
// https://leetcode.com/problems/count-collisions-on-a-road/

struct Solution;

impl Solution {
    pub fn count_collisions(directions: String) -> i32 {
        let chars: Vec<char> = directions.chars().collect();

        // Count total 'R' and 'L' cars
        let total_moving: i32 = chars.iter().filter(|&&c| c == 'R' || c == 'L').count() as i32;

        // Count leading 'L's that escape to the left
        let mut leading_l = 0;
        for &c in &chars {
            if c == 'L' {
                leading_l += 1;
            } else {
                break;
            }
        }

        // Count trailing 'R's that escape to the right
        let mut trailing_r = 0;
        for &c in chars.iter().rev() {
            if c == 'R' {
                trailing_r += 1;
            } else {
                break;
            }
        }

        // Collisions = all moving cars minus those that escape
        total_moving - leading_l - trailing_r
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_collisions_on_a_road::Solution;

    #[test]
    fn test_count_collisions_1() {
        let directions = "RLRSLL".to_string();
        assert_eq!(5, Solution::count_collisions(directions));
    }

    #[test]
    fn test_count_collisions_2() {
        let directions = "LLRR".to_string();
        assert_eq!(0, Solution::count_collisions(directions));
    }
}
