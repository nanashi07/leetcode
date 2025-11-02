// 1394. Find Lucky Integer in an Array
// https://leetcode.com/problems/find-lucky-integer-in-an-array/

struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        println!("arr: {:?}", arr);
        let mut arr = arr;
        arr.sort();
        let mut luck_num = -1;
        let mut count = 0;
        let mut last_item: Option<i32> = None;
        for i in 0..arr.len() {
            if Some(arr[i]) == last_item {
                count += 1;
            } else {
                if i > 0 && count == arr[i - 1] {
                    luck_num = count;
                }
                count = 1;
                last_item = Some(arr[i]);
            }
        }

        if count == arr[arr.len() - 1] {
            luck_num = count;
        }

        luck_num
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
