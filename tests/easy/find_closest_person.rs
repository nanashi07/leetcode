// 3516. Find Closest Person
// https://leetcode.com/problems/find-closest-person/

struct Solution;

impl Solution {
    pub fn find_closest(x: i32, y: i32, z: i32) -> i32 {
        println!("x: {x}, y: {y}, z: {z}");

        let xz = (z - x).abs();
        let yz = (z - y).abs();

        println!("xz: {xz}, yz: {yz}");

        if xz == yz {
            0
        } else if xz < yz {
            1
        } else {
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::easy::find_closest_person::Solution;

    #[test]
    fn test_find_closest_1() {
        let x = 2;
        let y = 7;
        let z = 4;
        assert_eq!(1, Solution::find_closest(x, y, z));
    }

    #[test]
    fn test_find_closest_2() {
        let x = 2;
        let y = 5;
        let z = 6;
        assert_eq!(2, Solution::find_closest(x, y, z));
    }

    #[test]
    fn test_find_closest_3() {
        let x = 1;
        let y = 5;
        let z = 3;
        assert_eq!(0, Solution::find_closest(x, y, z));
    }
}
