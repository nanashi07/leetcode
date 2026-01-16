// 2975. Maximum Square Area by Removing Fences From a Field
// https://leetcode.com/problems/maximum-square-area-by-removing-fences-from-a-field/

struct Solution;

impl Solution {
    pub fn maximize_square_area(m: i32, n: i32, h_fences: Vec<i32>, v_fences: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_square_area_by_removing_fences_from_a_field::Solution;

    #[test]
    fn test_maximize_square_area_1() {
        let m = 4;
        let n = 3;
        let h_fences = [2, 3].to_vec();
        let v_fences = [2].to_vec();
        assert_eq!(4, Solution::maximize_square_area(m, n, h_fences, v_fences));
    }

    #[test]
    fn test_maximize_square_area_2() {
        let m = 6;
        let n = 7;
        let h_fences = [2].to_vec();
        let v_fences = [4].to_vec();
        assert_eq!(-1, Solution::maximize_square_area(m, n, h_fences, v_fences));
    }
}
