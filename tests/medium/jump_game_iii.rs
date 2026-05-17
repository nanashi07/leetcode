// 1306. Jump Game III
// https://leetcode.com/problems/jump-game-iii/

struct Solution;

impl Solution {
    pub fn can_reach(mut arr: Vec<i32>, start: i32) -> bool {
        let n = arr.len();
        let mut stack = vec![start as usize];

        while let Some(curr) = stack.pop() {
            if arr[curr] == 0 {
                return true;
            }
            if arr[curr] < 0 {
                continue;
            }

            let jump = arr[curr];
            arr[curr] = -1; // mark as visited

            if curr as i32 + jump < n as i32 {
                stack.push((curr as i32 + jump) as usize);
            }
            if curr as i32 - jump >= 0 {
                stack.push((curr as i32 - jump) as usize);
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::jump_game_iii::Solution;

    #[test]
    fn test_can_reach_1() {
        let arr = [4, 2, 3, 0, 3, 1, 2].to_vec();
        let start = 5;
        assert_eq!(true, Solution::can_reach(arr, start));
    }

    #[test]
    fn test_can_reach_2() {
        let arr = [4, 2, 3, 0, 3, 1, 2].to_vec();
        let start = 0;
        assert_eq!(true, Solution::can_reach(arr, start));
    }

    #[test]
    fn test_can_reach_3() {
        let arr = [3, 0, 2, 1, 2].to_vec();
        let start = 2;
        assert_eq!(false, Solution::can_reach(arr, start));
    }
}
