// 3433. Count Mentions Per User
// https://leetcode.com/problems/count-mentions-per-user/

struct Solution;

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::count_mentions_per_user::Solution;

    #[test]
    fn test_count_mentions_1() {
        let number_of_users = 2;
        let events = [
            ["MESSAGE", "10", "id1 id0"],
            ["OFFLINE", "11", "0"],
            ["MESSAGE", "71", "HERE"],
        ]
        .iter()
        .map(|l| l.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
        let output = [2, 2].to_vec();
        assert_eq!(output, Solution::count_mentions(number_of_users, events));
    }

    #[test]
    fn test_count_mentions_2() {
        let number_of_users = 2;
        let events = [
            ["MESSAGE", "10", "id1 id0"],
            ["OFFLINE", "11", "0"],
            ["MESSAGE", "12", "ALL"],
        ]
        .iter()
        .map(|l| l.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
        let output = [2, 2].to_vec();
        assert_eq!(output, Solution::count_mentions(number_of_users, events));
    }

    #[test]
    fn test_count_mentions_3() {
        let number_of_users = 2;
        let events = [["OFFLINE", "10", "0"], ["MESSAGE", "12", "HERE"]]
            .iter()
            .map(|l| l.iter().map(|s| s.to_string()).collect::<Vec<_>>())
            .collect::<Vec<_>>();
        let output = [0, 1].to_vec();
        assert_eq!(output, Solution::count_mentions(number_of_users, events));
    }
}
