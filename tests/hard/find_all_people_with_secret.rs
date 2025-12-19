// 2092. Find All People With Secret
// https://leetcode.com/problems/find-all-people-with-secret/

struct Solution;

impl Solution {
    pub fn find_all_people(n: i32, meetings: Vec<Vec<i32>>, first_person: i32) -> Vec<i32> {
        todo!()
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
