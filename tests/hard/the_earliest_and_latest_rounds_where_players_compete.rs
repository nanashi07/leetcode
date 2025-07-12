// # 1900. The Earliest and Latest Rounds Where Players Compete
// https://leetcode.com/problems/the-earliest-and-latest-rounds-where-players-compete/

struct Solution;

impl Solution {
    pub fn earliest_and_latest(n: i32, first_player: i32, second_player: i32) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::the_earliest_and_latest_rounds_where_players_compete::Solution;

    #[test]
    fn test_earliest_and_latest_1() {
        let n = 11;
        let first_player = 2;
        let second_player = 4;
        assert_eq!(
            vec![3, 4],
            Solution::earliest_and_latest(n, first_player, second_player)
        );
    }

    #[test]
    fn test_earliest_and_latest_2() {
        let n = 5;
        let first_player = 1;
        let second_player = 5;
        assert_eq!(
            vec![1, 1],
            Solution::earliest_and_latest(n, first_player, second_player)
        );
    }
}
