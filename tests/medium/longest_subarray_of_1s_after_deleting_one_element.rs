// # 1493. Longest Subarray of 1's After Deleting One Element
// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/

struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        println!("nums: {:?}", &nums);

        let mut all_ones = nums[0];
        let mut vs = vec![];
        for i in 0..nums.len() {
            if vs.is_empty() {
                if nums[i] == 1 {
                    vs.push(vec![nums[i]]);
                } else {
                    vs.push(vec![]);
                }
            } else {
                let n = nums[i];
                let p = nums[i - 1];
                if n == 1 {
                    if p == 1 {
                        let len = vs.len();
                        vs[len - 1].push(n);
                    } else {
                        vs.push(vec![n]);
                    }
                } else {
                    all_ones = 0;
                    vs.push(vec![]);
                }
            }
        }
        println!("vs: {:?}", &vs);

        let mut r = vs[0].len();
        for i in 1..vs.len() {
            if i + 1 < vs.len() {
                r = r.max(vs[i].len() + vs[i + 1].len());
                if vs[i].len() == 0 {
                    r = r.max(vs[i - 1].len() + vs[i + 1].len());
                }
            }
            r = r.max(vs[i - 1].len() + vs[i].len())
        }

        r as i32 - all_ones
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::longest_subarray_of_1s_after_deleting_one_element::Solution;

    #[test]
    fn test_longest_subarray_1() {
        let nums = [1, 1, 0, 1].to_vec();
        assert_eq!(3, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_2() {
        let nums = [0, 1, 1, 1, 0, 1, 1, 0, 1].to_vec();
        assert_eq!(5, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_3() {
        let nums = [1, 1, 1].to_vec();
        assert_eq!(2, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_4() {
        let nums = [1, 1, 0, 0, 1, 1, 1, 0, 1].to_vec();
        assert_eq!(4, Solution::longest_subarray(nums));
    }

    #[test]
    fn test_longest_subarray_5() {
        let nums = [0, 0, 0].to_vec();
        assert_eq!(0, Solution::longest_subarray(nums));
    }
}
