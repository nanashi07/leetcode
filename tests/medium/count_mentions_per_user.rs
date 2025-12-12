// 3433. Count Mentions Per User
// https://leetcode.com/problems/count-mentions-per-user/

struct Solution;

impl Solution {
    pub fn count_mentions(number_of_users: i32, events: Vec<Vec<String>>) -> Vec<i32> {
        let mut counts: Vec<i32> = vec![0; number_of_users as usize];
        let mut offline_until: Vec<i64> = vec![i64::MIN; number_of_users as usize];

        let mut events = events;
        events.sort_by(|a, b| {
            let ta = a[1].parse::<i64>().unwrap_or(0);
            let tb = b[1].parse::<i64>().unwrap_or(0);
            if ta != tb {
                ta.cmp(&tb)
            } else {
                // OFFLINE and ONLINE before MESSAGE at same time
                match (a[0].as_str(), b[0].as_str()) {
                    ("OFFLINE", "MESSAGE") | ("ONLINE", "MESSAGE") => std::cmp::Ordering::Less,
                    ("MESSAGE", "OFFLINE") | ("MESSAGE", "ONLINE") => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            }
        });

        for event in events {
            if event.len() < 3 {
                continue;
            }
            let event_type = &event[0];
            let time: i64 = event[1].parse().unwrap_or(0);
            if event_type == "MESSAGE" {
                let mentions = &event[2];
                let mut mentioned: Vec<i32> = vec![0; number_of_users as usize];
                if mentions == "ALL" {
                    for i in 0..number_of_users as usize {
                        mentioned[i] = 1;
                    }
                } else if mentions == "HERE" {
                    for i in 0..number_of_users as usize {
                        if offline_until[i] <= time {
                            mentioned[i] = 1;
                        }
                    }
                } else {
                    for part in mentions.split_whitespace() {
                        if let Some(id_str) = part.strip_prefix("id") {
                            if let Ok(id) = id_str.parse::<usize>() {
                                if id < number_of_users as usize {
                                    mentioned[id] += 1;
                                }
                            }
                        }
                    }
                }
                for i in 0..number_of_users as usize {
                    counts[i] += mentioned[i];
                }
            } else if event_type == "OFFLINE" {
                let user: usize = event[2].parse().unwrap_or(0);
                if user < number_of_users as usize {
                    offline_until[user] = time + 60;
                }
            } else if event_type == "ONLINE" {
                let user: usize = event[2].parse().unwrap_or(0);
                if user < number_of_users as usize {
                    offline_until[user] = i64::MIN;
                }
            }
        }

        counts
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

    #[test]
    fn test_count_mentions_4() {
        let number_of_users = 3;
        let events = [
            ["MESSAGE", "2", "HERE"],
            ["OFFLINE", "2", "1"],
            ["OFFLINE", "1", "0"],
            ["MESSAGE", "61", "HERE"],
        ]
        .iter()
        .map(|l| l.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
        let output = [1, 0, 2].to_vec();
        assert_eq!(output, Solution::count_mentions(number_of_users, events));
    }

    #[test]
    fn test_count_mentions_5() {
        let number_of_users = 5;
        let events = [
            ["OFFLINE", "28", "1"],
            ["OFFLINE", "14", "2"],
            ["MESSAGE", "2", "ALL"],
            ["MESSAGE", "28", "HERE"],
            ["OFFLINE", "66", "0"],
            ["MESSAGE", "34", "id2"],
            ["MESSAGE", "83", "HERE"],
            ["MESSAGE", "40", "id3 id3 id2 id4 id4"],
        ]
        .iter()
        .map(|l| l.iter().map(|s| s.to_string()).collect::<Vec<_>>())
        .collect::<Vec<_>>();
        let output = [2, 1, 4, 5, 5].to_vec();
        assert_eq!(output, Solution::count_mentions(number_of_users, events));
    }
}
