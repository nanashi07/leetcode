// 679. 24 Game
// https://leetcode.com/problems/24-game/

struct Solution;

impl Solution {
    pub fn judge_point24(cards: Vec<i32>) -> bool {
        let mut nums: Vec<f64> = cards.into_iter().map(|x| x as f64).collect();
        Self::backtrack(&mut nums)
    }

    fn backtrack(nums: &mut Vec<f64>) -> bool {
        const EPS: f64 = 1e-6;

        // Base case: if only one number left, check if it's close to 24
        if nums.len() == 1 {
            return (nums[0] - 24.0).abs() < EPS;
        }

        // Try all pairs of numbers
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let a = nums[i];
                let b = nums[j];

                // Remove the two numbers we're combining
                let mut remaining = Vec::new();
                for k in 0..nums.len() {
                    if k != i && k != j {
                        remaining.push(nums[k]);
                    }
                }

                // Try all possible operations
                let operations = [a + b, a - b, b - a, a * b];

                for result in operations {
                    remaining.push(result);
                    if Self::backtrack(&mut remaining) {
                        return true;
                    }
                    remaining.pop();
                }

                // Division operations (check for division by zero)
                if b.abs() > EPS {
                    remaining.push(a / b);
                    if Self::backtrack(&mut remaining) {
                        return true;
                    }
                    remaining.pop();
                }

                if a.abs() > EPS {
                    remaining.push(b / a);
                    if Self::backtrack(&mut remaining) {
                        return true;
                    }
                    remaining.pop();
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::twenty_four_game::Solution;

    #[test]
    fn test_judge_point24_1() {
        let cards = [4, 1, 8, 7].to_vec();
        assert_eq!(true, Solution::judge_point24(cards));
    }

    #[test]
    fn test_judge_point24_2() {
        let cards = [1, 2, 1, 2].to_vec();
        assert_eq!(false, Solution::judge_point24(cards));
    }
}
