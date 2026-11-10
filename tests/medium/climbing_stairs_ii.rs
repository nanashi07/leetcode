// 3693. Climbing Stairs II
// https://leetcode.com/problems/climbing-stairs-ii/

struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::climbing_stairs_ii::Solution;

    #[test]
    fn test_climb_stairs_1() {
        let n = 4;
        let costs = [1, 2, 3, 4].to_vec();
        assert_eq!(13, Solution::climb_stairs(n, costs));
    }

    #[test]
    fn test_climb_stairs_2() {
        let n = 4;
        let costs = [5, 1, 6, 2].to_vec();
        assert_eq!(11, Solution::climb_stairs(n, costs));
    }

    #[test]
    fn test_climb_stairs_3() {
        let n = 3;
        let costs = [9, 8, 3].to_vec();
        assert_eq!(12, Solution::climb_stairs(n, costs));
    }
}
