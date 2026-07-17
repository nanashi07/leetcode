// 1331. Rank Transform of an Array
// https://leetcode.com/problems/rank-transform-of-an-array/

struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        let mut s = arr.clone();
        s.sort_unstable();
        s.dedup();
        let mut arr = arr;
        for item in &mut arr {
            let r = s.binary_search(item).unwrap();
            *item = r as i32 + 1;
        }
        arr
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::rank_transform_of_an_array::Solution;

    #[test]
    fn test_array_rank_transform_1() {
        let arr = [40, 10, 20, 30].to_vec();
        assert_eq!([4, 1, 2, 3].to_vec(), Solution::array_rank_transform(arr));
    }

    #[test]
    fn test_array_rank_transform_2() {
        let arr = [100, 100, 100].to_vec();
        assert_eq!([1, 1, 1].to_vec(), Solution::array_rank_transform(arr));
    }

    #[test]
    fn test_array_rank_transform_3() {
        let arr = [37, 12, 28, 9, 100, 56, 80, 5, 12].to_vec();
        assert_eq!(
            [5, 3, 4, 2, 8, 6, 7, 1, 3].to_vec(),
            Solution::array_rank_transform(arr)
        );
    }
}
