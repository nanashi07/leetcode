// 1437. Check If All 1's Are at Least Length K Places Away
// https://leetcode.com/problems/check-if-all-1s-are-at-least-length-k-places-away/

struct Solution;

impl Solution {
    pub fn k_length_apart(nums: Vec<i32>, k: i32) -> bool {
        println!("nums: {:?}, k: {k}", &nums);

        let mut space = i32::MAX;
        let mut counter = -1;
        for i in 0..nums.len() {
            let n = &nums[i];
            if *n == 1 {
                if counter > -1 {
                    space = space.min(counter);
                }
                counter = 0;
            } else if counter != -1 {
                counter += 1;
            }
        }

        space >= k
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::check_if_all_1s_are_at_least_length_k_places_away::Solution;

    #[test]
    fn test_k_length_apart_1() {
        let nums = [1, 0, 0, 0, 1, 0, 0, 1].to_vec();
        let k = 2;
        assert_eq!(true, Solution::k_length_apart(nums, k));
    }

    #[test]
    fn test_k_length_apart_2() {
        let nums = [1, 0, 0, 1, 0, 1].to_vec();
        let k = 2;
        assert_eq!(false, Solution::k_length_apart(nums, k));
    }

    #[test]
    fn test_k_length_apart_3() {
        let nums = [1, 1, 1, 1, 1].to_vec();
        let k = 0;
        assert_eq!(true, Solution::k_length_apart(nums, k));
    }

    #[test]
    fn test_k_length_apart_4() {
        let nums = [1, 0, 0, 0, 1, 0, 0, 1, 0].to_vec();
        let k = 2;
        assert_eq!(true, Solution::k_length_apart(nums, k));
    }

    #[test]
    fn test_k_length_apart_5() {
        let nums = [0, 1, 0, 1].to_vec();
        let k = 1;
        assert_eq!(true, Solution::k_length_apart(nums, k));
    }
}
