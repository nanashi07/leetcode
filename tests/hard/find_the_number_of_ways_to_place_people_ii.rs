// # 3027. Find the Number of Ways to Place People II
// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-ii/

struct Solution;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_the_number_of_ways_to_place_people_ii::Solution;

    #[test]
    fn test_number_of_pairs_1() {
        let points = [[1, 1], [2, 2], [3, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(0, Solution::number_of_pairs(points));
    }

    #[test]
    fn test_number_of_pairs_2() {
        let points = [[6, 2], [4, 4], [2, 6]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(2, Solution::number_of_pairs(points));
    }

    #[test]
    fn test_number_of_pairs_3() {
        let points = [[3, 1], [1, 3], [1, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(2, Solution::number_of_pairs(points));
    }
}
