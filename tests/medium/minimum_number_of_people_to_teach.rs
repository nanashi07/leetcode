// # 1733. Minimum Number of People to Teach
// https://leetcode.com/problems/minimum-number-of-people-to-teach/

struct Solution;

impl Solution {
    pub fn minimum_teachings(n: i32, languages: Vec<Vec<i32>>, friendships: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::minimum_number_of_people_to_teach::Solution;

    #[test]
    fn test_minimum_teachings_1() {
        let n = 2;
        let languages = vec![vec![1], vec![2], vec![1, 2]];
        let friendships = [[1, 2], [1, 3], [2, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(1, Solution::minimum_teachings(n, languages, friendships));
    }

    #[test]
    fn test_minimum_teachings_2() {
        let n = 3;
        let languages = vec![vec![2], vec![1, 3], vec![1, 2], vec![3]];
        let friendships = [[1, 4], [1, 2], [3, 4], [2, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(2, Solution::minimum_teachings(n, languages, friendships));
    }
}
