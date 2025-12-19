// 2092. Find All People With Secret
// https://leetcode.com/problems/find-all-people-with-secret/

use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn find_all_people(_n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        // Group meetings by time
        let mut meetings_by_time: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
        for meeting in meetings.iter() {
            let (person1, person2, time) = (meeting[0], meeting[1], meeting[2]);
            meetings_by_time
                .entry(time)
                .or_insert(Vec::new())
                .push((person1, person2));
        }

        // Sort times
        let mut times: Vec<i32> = meetings_by_time.keys().copied().collect();
        times.sort_unstable();

        // Track who knows the secret
        let mut knows_secret: HashSet<i32> = HashSet::new();
        knows_secret.insert(0);
        knows_secret.insert(first_person);

        // Process meetings at each time
        for time in times {
            let meetings_at_time = meetings_by_time.get(&time).unwrap();

            // Build graph for this time slot
            let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
            let mut people_at_meeting: HashSet<i32> = HashSet::new();

            for &(person1, person2) in meetings_at_time {
                graph.entry(person1).or_insert(Vec::new()).push(person2);
                graph.entry(person2).or_insert(Vec::new()).push(person1);
                people_at_meeting.insert(person1);
                people_at_meeting.insert(person2);
            }

            // Find all people who know secret at this time (before meetings)
            let initial_knows: Vec<i32> = people_at_meeting
                .iter()
                .filter(|&&p| knows_secret.contains(&p))
                .copied()
                .collect();

            // BFS/DFS to spread secret through connected components
            let mut visited: HashSet<i32> = HashSet::new();

            for start in initial_knows {
                if visited.contains(&start) {
                    continue;
                }

                // BFS from this person
                let mut queue: Vec<i32> = vec![start];
                visited.insert(start);

                while let Some(person) = queue.pop() {
                    knows_secret.insert(person);

                    if let Some(neighbors) = graph.get(&person) {
                        for &neighbor in neighbors {
                            if !visited.contains(&neighbor) {
                                visited.insert(neighbor);
                                queue.push(neighbor);
                            }
                        }
                    }
                }
            }
        }

        // Collect and sort result
        let mut result: Vec<i32> = knows_secret.into_iter().collect();
        result.sort_unstable();
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_all_people_with_secret::Solution;

    #[test]
    fn test_find_all_people_1() {
        let n = 6;
        let meetings = [[1, 2, 5], [2, 3, 8], [1, 5, 10]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let first_person = 1;
        let output = [0, 1, 2, 3, 5].to_vec();
        assert_eq!(output, Solution::find_all_people(n, meetings, first_person));
    }

    #[test]
    fn test_find_all_people_2() {
        let n = 4;
        let meetings = [[3, 1, 3], [1, 2, 2], [0, 3, 3]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let first_person = 3;
        let output = [0, 1, 3].to_vec();
        assert_eq!(output, Solution::find_all_people(n, meetings, first_person));
    }

    #[test]
    fn test_find_all_people_3() {
        let n = 5;
        let meetings = [[3, 4, 2], [1, 2, 1], [2, 3, 1]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        let first_person = 1;
        let output = [0, 1, 2, 3, 4].to_vec();
        assert_eq!(output, Solution::find_all_people(n, meetings, first_person));
    }
}
