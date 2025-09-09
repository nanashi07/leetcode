// # 2327. Number of People Aware of a Secret
// https://leetcode.com/problems/number-of-people-aware-of-a-secret/

struct Solution;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::number_of_people_aware_of_a_secret::Solution;

    #[test]
    fn test_people_aware_of_secret_1() {
        let n = 6;
        let delay = 2;
        let forget = 4;
        assert_eq!(5, Solution::people_aware_of_secret(n, delay, forget));
    }

    #[test]
    fn test_people_aware_of_secret_2() {
        let n = 4;
        let delay = 1;
        let forget = 3;
        assert_eq!(6, Solution::people_aware_of_secret(n, delay, forget));
    }
}
