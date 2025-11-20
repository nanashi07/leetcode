// 757. Set Intersection Size At Least Two
// https://leetcode.com/problems/set-intersection-size-at-least-two/

struct Solution;

impl Solution {
    pub fn intersection_size_two(mut intervals: Vec<Vec<i32>>) -> i32 {
        // Sort by end point, then by start point descending
        // This ensures we process intervals that end earlier first
        // and prefer larger intervals when ends are the same
        intervals.sort_by(|a, b| {
            if a[1] != b[1] {
                a[1].cmp(&b[1])
            } else {
                b[0].cmp(&a[0])
            }
        });

        let mut result = 0;
        let mut p1 = -1; // largest element in our set
        let mut p2 = -1; // second largest element in our set

        for interval in intervals {
            let start = interval[0];
            let end = interval[1];

            // Check how many elements from {p2, p1} are in [start, end]
            let mut count = 0;
            if p2 >= start {
                count += 1;
            }
            if p1 >= start {
                count += 1;
            }

            if count == 0 {
                // Need to add 2 elements, choose end-1 and end
                p2 = end - 1;
                p1 = end;
                result += 2;
            } else if count == 1 {
                // Need to add 1 element
                // Add the largest possible (end), unless p1 == end, then add end-1
                if p1 == end {
                    p2 = p1;
                    p1 = end - 1;
                } else {
                    p2 = p1;
                    p1 = end;
                }
                result += 1;
            }
            // If count == 2, we already have enough elements in this interval
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::set_intersection_size_at_least_two::Solution;

    #[test]
    fn test_intersection_size_two_1() {
        let intervals = [[1, 3], [3, 7], [8, 9]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(5, Solution::intersection_size_two(intervals));
    }

    #[test]
    fn test_intersection_size_two_2() {
        let intervals = [[1, 3], [1, 4], [2, 5], [3, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(3, Solution::intersection_size_two(intervals));
    }

    #[test]
    fn test_intersection_size_two_3() {
        let intervals = [[1, 2], [2, 3], [2, 4], [4, 5]]
            .iter()
            .map(|l| l.to_vec())
            .collect::<Vec<_>>();
        assert_eq!(5, Solution::intersection_size_two(intervals));
    }
}
