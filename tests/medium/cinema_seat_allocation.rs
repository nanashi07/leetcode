// 1386. Cinema Seat Allocation
// https://leetcode.com/problems/cinema-seat-allocation/

struct Solution;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::cinema_seat_allocation::Solution;
    use crate::shared::vec2d::to_vec2d;

    #[test]
    fn test_max_number_of_families_1() {
        let n = 3;
        let reserved_seats = to_vec2d([[1, 2], [1, 3], [1, 8], [2, 6], [3, 1], [3, 10]]);
        assert_eq!(4, Solution::max_number_of_families(n, reserved_seats));
    }

    #[test]
    fn test_max_number_of_families_2() {
        let n = 2;
        let reserved_seats = to_vec2d([[2, 1], [1, 8], [2, 6]]);
        assert_eq!(2, Solution::max_number_of_families(n, reserved_seats));
    }

    #[test]
    fn test_max_number_of_families_3() {
        let n = 4;
        let reserved_seats = to_vec2d([[4, 3], [1, 4], [4, 6], [1, 7]]);
        assert_eq!(4, Solution::max_number_of_families(n, reserved_seats));
    }
}
