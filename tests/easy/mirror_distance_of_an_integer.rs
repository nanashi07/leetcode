// 3783. Mirror Distance of an Integer
// https://leetcode.com/problems/mirror-distance-of-an-integer/

struct Solution;

impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::mirror_distance_of_an_integer::Solution;

    #[test]
    fn test_mirror_distance_1() {
        let n = 25;
        assert_eq!(27, Solution::mirror_distance(n));
    }

    #[test]
    fn test_mirror_distance_2() {
        let n = 10;
        assert_eq!(9, Solution::mirror_distance(n));
    }

    #[test]
    fn test_mirror_distance_3() {
        let n = 7;
        assert_eq!(0, Solution::mirror_distance(n));
    }
}
