// # 2402. Meeting Rooms III
// https://leetcode.com/problems/meeting-rooms-iii/

use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn most_booked(n: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut meetings = meetings;

        // Sort meetings by start time
        meetings.sort_by_key(|m| m[0]);

        // Count how many meetings each room hosts
        let mut room_count = vec![0; n];

        // Min heap for available rooms (by room number)
        let mut available_rooms: BinaryHeap<Reverse<usize>> = (0..n).map(Reverse).collect();

        // Min heap for busy rooms: (end_time, room_number)
        let mut busy_rooms: BinaryHeap<Reverse<(i64, usize)>> = BinaryHeap::new();

        for meeting in meetings {
            let start = meeting[0] as i64;
            let end = meeting[1] as i64;
            let duration = end - start;

            // Free up rooms that have finished their meetings
            while let Some(Reverse((end_time, _room))) = busy_rooms.peek() {
                if *end_time <= start {
                    let (_, room) = busy_rooms.pop().unwrap().0;
                    available_rooms.push(Reverse(room));
                } else {
                    break;
                }
            }

            let room_to_use;
            let actual_end_time;

            if let Some(Reverse(room)) = available_rooms.pop() {
                // Room is available immediately
                room_to_use = room;
                actual_end_time = end;
            } else {
                // All rooms are busy, use the one that finishes earliest
                let Reverse((earliest_end, room)) = busy_rooms.pop().unwrap();
                room_to_use = room;
                // Meeting starts when the room becomes available
                actual_end_time = earliest_end + duration;
            }

            // Book the room
            room_count[room_to_use] += 1;
            busy_rooms.push(Reverse((actual_end_time, room_to_use)));
        }

        // Find the room with the most bookings (smallest index in case of tie)
        let mut max_count = 0;
        let mut result = 0;

        for (room, &count) in room_count.iter().enumerate() {
            if count > max_count {
                max_count = count;
                result = room;
            }
        }

        result as i32
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
