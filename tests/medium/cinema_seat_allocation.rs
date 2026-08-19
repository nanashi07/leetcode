// 1386. Cinema Seat Allocation
// https://leetcode.com/problems/cinema-seat-allocation/

use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        const LEFT: u8 = 0b0000_1111;
        const MIDDLE: u8 = 0b0011_1100;
        const RIGHT: u8 = 0b1111_0000;

        let mut reserved_by_row = HashMap::new();
        for seat in reserved_seats {
            let row = seat[0];
            let number = seat[1];
            if (2..=9).contains(&number) {
                *reserved_by_row.entry(row).or_default() |= 1 << (number - 2);
            }
        }

        let mut families = 2 * n;
        for mask in reserved_by_row.values() {
            let left_available = mask & LEFT == 0;
            let middle_available = mask & MIDDLE == 0;
            let right_available = mask & RIGHT == 0;
            let row_families = if left_available && right_available {
                2
            } else if left_available || middle_available || right_available {
                1
            } else {
                0
            };
            families -= 2 - row_families;
        }
        families
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
