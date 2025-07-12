// # 2402. Meeting Rooms III
// https://leetcode.com/problems/meeting-rooms-iii/

struct Solution;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut rooms_use_count = vec![0; n as usize];
        let mut rooms_use = vec![0; n as usize];
        let mut meetings = meetings;

        for m in 0..meetings.len() {
            let mut meeting = &meetings[m];

            for r in 0..rooms_use.len() {
                if rooms_use[r] > meeting[0] {
                    // next or
                }
            }
        }

        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::meeting_rooms_iii::Solution;

    #[test]
    fn test_most_booked_1() {
        let n = 2;
        let meetings = [[0, 10], [1, 5], [2, 7], [3, 4]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(0, Solution::most_booked(n, meetings));
    }

    #[test]
    fn test_most_booked_2() {
        let n = 3;
        let meetings = [[1, 20], [2, 10], [3, 5], [4, 9], [6, 8]]
            .iter()
            .map(|e| e.to_vec())
            .collect::<Vec<Vec<i32>>>();
        assert_eq!(1, Solution::most_booked(n, meetings));
    }
}
