// 3025. Find the Number of Ways to Place People I
// https://leetcode.com/problems/find-the-number-of-ways-to-place-people-i/

struct Solution;

impl Solution {
    pub fn number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut count = 0;

        // For each pair of points (i, j), check if point i can be the upper-left
        // and point j can be the lower-right of a rectangle with no other points inside
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }

                let (x1, y1) = (points[i][0], points[i][1]);
                let (x2, y2) = (points[j][0], points[j][1]);

                // Point i should be upper-left of point j
                // This means x1 <= x2 and y1 >= y2
                if x1 <= x2 && y1 >= y2 {
                    // Check if there are any other points inside or on the boundary
                    // of the rectangle formed by these two points
                    let mut has_point_inside = false;

                    for k in 0..n {
                        if k == i || k == j {
                            continue;
                        }

                        let (x3, y3) = (points[k][0], points[k][1]);

                        // Check if point k is inside or on the rectangle boundary
                        // Rectangle boundaries: x1 <= x <= x2, y2 <= y <= y1
                        if x1 <= x3 && x3 <= x2 && y2 <= y3 && y3 <= y1 {
                            has_point_inside = true;
                            break;
                        }
                    }

                    if !has_point_inside {
                        count += 1;
                    }
                }
            }
        }

        count
    }

    // failed
    pub fn _number_of_pairs(points: Vec<Vec<i32>>) -> i32 {
        let mut points = points;
        points.sort_unstable_by(|a, b| (a[0], a[1]).cmp(&(b[0], b[1])));

        println!("points: {:?}", &points);
        let len = points.len();
        let mut count = 0;

        for i in 0..len - 1 {
            for j in i + 1..len {
                let (a, b) = (&points[i], &points[j]);

                let (a, b) = if b[0] == a[0] && a[1] < b[1] {
                    println!("swap");
                    (&points[j], &points[i])
                } else {
                    (&points[i], &points[j])
                };

                println!("A: {:?}, B: {:?}", a, b);
                if b[0] >= a[0] && a[1] >= b[1] {
                    let mut contains = false;

                    for m in i + 1..j {
                        let c = &points[m];
                        if c[0] > a[0] && c[0] < b[0] && c[1] < a[1] && c[1] > b[1] {
                            println!("A: {:?}, B: {:?}, C: {:?}", a, b, c);
                            contains = true;
                            break;
                        }
                    }

                    if !contains {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::find_the_number_of_ways_to_place_people_i::Solution;

    #[test]
    fn test_number_of_pairs_1() {
        let points = [[1, 1], [2, 2], [3, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(0, Solution::number_of_pairs(points));
    }

    #[test]
    fn test_number_of_pairs_2() {
        let points = [[6, 2], [4, 4], [2, 6]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(2, Solution::number_of_pairs(points));
    }

    #[test]
    fn test_number_of_pairs_3() {
        let points = [[3, 1], [1, 3], [1, 1]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(2, Solution::number_of_pairs(points));
    }

    #[test]
    fn test_number_of_pairs_4() {
        let points = [[0, 0], [0, 3]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(1, Solution::number_of_pairs(points));
    }
}
