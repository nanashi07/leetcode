// 2410. Maximum Matching of Players With Trainers
// https://leetcode.com/problems/maximum-matching-of-players-with-trainers/

struct Solution;

impl Solution {
    pub fn match_players_and_trainers(players: Vec<i32>, trainers: Vec<i32>) -> i32 {
        let mut players = players;
        let mut trainers = trainers;
        players.sort_unstable();
        trainers.sort_unstable();

        let mut it = 0;
        let mut count = 0;

        for ip in 0..players.len() {
            while it < trainers.len() && trainers[it] < players[ip] {
                it = it + 1;
            }

            if it < trainers.len() && trainers[it] >= players[ip] {
                count += 1;
                it += 1;
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_matching_of_players_with_trainers::Solution;

    #[test]
    fn test_match_players_and_trainers_1() {
        let players = vec![4, 7, 9];
        let trainers = vec![8, 2, 5, 8];
        assert_eq!(2, Solution::match_players_and_trainers(players, trainers));
    }

    #[test]
    fn test_match_players_and_trainers_2() {
        let players = vec![1, 1, 1];
        let trainers = vec![10];
        assert_eq!(1, Solution::match_players_and_trainers(players, trainers));
    }
}
