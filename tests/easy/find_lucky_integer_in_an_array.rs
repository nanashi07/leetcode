// # 1394. Find Lucky Integer in an Array
// https://leetcode.com/problems/find-lucky-integer-in-an-array/

struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_lucky_integer_in_an_array::Solution;

    #[test]
    fn test_find_lucky_1() {
        let arr = [2, 2, 3, 4].to_vec();
        assert_eq!(2, Solution::find_lucky(arr));
    }

    #[test]
    fn test_find_lucky_2() {
        let arr = [1, 2, 2, 3, 3, 3].to_vec();
        assert_eq!(3, Solution::find_lucky(arr));
    }

    #[test]
    fn test_find_lucky_3() {
        let arr = [2, 2, 2, 3, 3].to_vec();
        assert_eq!(-1, Solution::find_lucky(arr));
    }
}
