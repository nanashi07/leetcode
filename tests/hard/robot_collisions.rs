// 2751. Robot Collisions
// https://leetcode.com/problems/robot-collisions/

struct Solution;

impl Solution {
    pub fn survived_robots_healths(
        positions: Vec<i32>,
        healths: Vec<i32>,
        directions: String,
    ) -> Vec<i32> {
        let n = positions.len();
        let dirs: Vec<u8> = directions.into_bytes();
        // sort indices by position
        let mut order: Vec<usize> = (0..n).collect();
        order.sort_unstable_by_key(|&i| positions[i]);

        let mut healths = healths;
        // stack holds indices (in position order) of rightward robots still alive
        let mut stack: Vec<usize> = Vec::with_capacity(n);

        for &i in &order {
            if dirs[i] == b'R' {
                stack.push(i);
            } else {
                // moving left: collide with rightward robots on stack
                let mut alive = true;
                while let Some(&top) = stack.last() {
                    if dirs[top] == b'L' {
                        break;
                    }
                    // top is moving R, current (i) is moving L
                    match healths[top].cmp(&healths[i]) {
                        std::cmp::Ordering::Greater => {
                            healths[top] -= 1;
                            healths[i] = 0;
                            alive = false;
                            break;
                        }
                        std::cmp::Ordering::Less => {
                            healths[top] = 0;
                            healths[i] -= 1;
                            stack.pop();
                        }
                        std::cmp::Ordering::Equal => {
                            healths[top] = 0;
                            healths[i] = 0;
                            stack.pop();
                            alive = false;
                            break;
                        }
                    }
                }
                if alive {
                    stack.push(i);
                }
            }
        }

        // collect survivors in original index order
        (0..n).filter_map(|i| if healths[i] > 0 { Some(healths[i]) } else { None }).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::robot_collisions::Solution;

    #[test]
    fn test_survived_robots_healths_1() {
        let positions = [5, 4, 3, 2, 1].to_vec();
        let healths = [2, 17, 9, 15, 10].to_vec();
        let directions = "RRRRR".to_string();
        assert_eq!(
            [2, 17, 9, 15, 10].to_vec(),
            Solution::survived_robots_healths(positions, healths, directions)
        );
    }

    #[test]
    fn test_survived_robots_healths_2() {
        let positions = [3, 5, 2, 6].to_vec();
        let healths = [10, 10, 15, 12].to_vec();
        let directions = "RLRL".to_string();
        assert_eq!(
            [14].to_vec(),
            Solution::survived_robots_healths(positions, healths, directions)
        );
    }

    #[test]
    fn test_survived_robots_healths_3() {
        let positions = [1, 2, 5, 6].to_vec();
        let healths = [10, 10, 11, 11].to_vec();
        let directions = "RLRL".to_string();
        assert_eq!(
            [0; 0].to_vec(),
            Solution::survived_robots_healths(positions, healths, directions)
        );
    }
}
