// 2409. Count Days Spent Together
// https://leetcode.com/problems/count-days-spent-together/

struct Solution;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::count_days_spent_together::Solution;

    #[test]
    fn test_count_days_together_1() {
        let arrive_alice = "08-15".to_string();
        let leave_alice = "08-18".to_string();
        let arrive_bob = "08-16".to_string();
        let leave_bob = "08-19".to_string();
        assert_eq!(
            3,
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob)
        );
    }

    #[test]
    fn test_count_days_together_2() {
        let arrive_alice = "10-01".to_string();
        let leave_alice = "10-31".to_string();
        let arrive_bob = "11-01".to_string();
        let leave_bob = "12-31".to_string();
        assert_eq!(
            0,
            Solution::count_days_together(arrive_alice, leave_alice, arrive_bob, leave_bob)
        );
    }
}
