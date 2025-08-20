// # 898. Bitwise ORs of Subarrays
// https://leetcode.com/problems/bitwise-ors-of-subarrays/

use std::collections::HashSet;

struct Solution;

impl Solution {
    // https://leetcode.com/problems/bitwise-ors-of-subarrays/editorial/
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        println!("arr: {:?}", &arr);

        let mut xors: HashSet<i32> = HashSet::new();
        let mut curr: HashSet<i32> = HashSet::from([0]); // init one

        for n in arr {
            let mut tmp: HashSet<i32> = HashSet::new();

            tmp.insert(n);
            // bitwise OR curr with n into a new XOR set
            for c in &curr {
                tmp.insert(*c | n);
            }
            println!("tmp: {:?}", &tmp);
            // re-assign for next round
            curr = tmp;

            for c in &curr {
                xors.insert(*c);
            }
        }

        xors.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::bitwise_ors_of_subarrays::Solution;

    #[test]
    fn test_subarray_bitwise_o_rs_1() {
        let arr = [0].to_vec();
        assert_eq!(1, Solution::subarray_bitwise_o_rs(arr));
    }

    #[test]
    fn test_subarray_bitwise_o_rs_2() {
        let arr = [1, 1, 2].to_vec();
        assert_eq!(3, Solution::subarray_bitwise_o_rs(arr));
    }

    #[test]
    fn test_subarray_bitwise_o_rs_3() {
        let arr = [1, 2, 4].to_vec();
        assert_eq!(6, Solution::subarray_bitwise_o_rs(arr));
    }
}
