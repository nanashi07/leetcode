// 2029. Stone Game IX
// https://leetcode.com/problems/stone-game-ix/

struct Solution;

impl Solution {
    pub fn stone_game_ix(stones: Vec<i32>) -> bool {
        let mut counts = [0usize; 3];
        for stone in stones {
            counts[(stone % 3) as usize] += 1;
        }

        if counts[0] % 2 == 0 {
            counts[1] > 0 && counts[2] > 0
        } else {
            counts[1].abs_diff(counts[2]) > 2
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::stone_game_ix::Solution;

    #[test]
    fn test_stone_game_ix_1() {
        let stones = [2, 1].to_vec();
        assert!(Solution::stone_game_ix(stones));
    }

    #[test]
    fn test_stone_game_ix_2() {
        let stones = [2].to_vec();
        assert!(!Solution::stone_game_ix(stones));
    }

    #[test]
    fn test_stone_game_ix_3() {
        let stones = [5, 1, 2, 4, 3].to_vec();
        assert!(!Solution::stone_game_ix(stones));
    }
}
