// 2409. Count Days Spent Together
// https://leetcode.com/problems/count-days-spent-together/

/// Days elapsed before the first day of each month in a non-leap year.
const CUM_DAYS_BEFORE_MONTH: [i32; 12] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

struct Solution;

impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        let start = Self::day_of_year(&arrive_alice).max(Self::day_of_year(&arrive_bob));
        let end = Self::day_of_year(&leave_alice).min(Self::day_of_year(&leave_bob));

        (end - start + 1).max(0)
    }

    /// Converts a `"MM-DD"` date of a non-leap year into its 1-based day of the year.
    fn day_of_year(date: &str) -> i32 {
        let bytes = date.as_bytes();
        let month = (bytes[0] - b'0') as usize * 10 + (bytes[1] - b'0') as usize;
        let day = (bytes[3] - b'0') as i32 * 10 + (bytes[4] - b'0') as i32;

        CUM_DAYS_BEFORE_MONTH[month - 1] + day
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
