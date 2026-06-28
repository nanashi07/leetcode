// 1846. Maximum Element After Decreasing and Rearranging
// https://leetcode.com/problems/maximum-element-after-decreasing-and-rearranging/

struct Solution;

impl Solution {
    pub fn maximum_element_after_decrementing_and_rearranging(arr: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_element_after_decreasing_and_rearranging::Solution;

    #[test]
    fn test_maximum_element_after_decrementing_and_rearranging_1() {
        let arr = [2, 2, 1, 2, 1].to_vec();
        assert_eq!(
            2,
            Solution::maximum_element_after_decrementing_and_rearranging(arr)
        );
    }

    #[test]
    fn test_maximum_element_after_decrementing_and_rearranging_2() {
        let arr = [100, 1, 1000].to_vec();
        assert_eq!(
            3,
            Solution::maximum_element_after_decrementing_and_rearranging(arr)
        );
    }

    #[test]
    fn test_maximum_element_after_decrementing_and_rearranging_3() {
        let arr = [1, 2, 3, 4, 5].to_vec();
        assert_eq!(
            5,
            Solution::maximum_element_after_decrementing_and_rearranging(arr)
        );
    }
}
