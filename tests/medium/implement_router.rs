// # 3508. Implement Router
// https://leetcode.com/problems/implement-router/
use std::cell::RefCell;
use std::collections::{HashMap, HashSet, VecDeque};

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */
struct Router {
    memory_limit: i32,
    packets: RefCell<VecDeque<(i32, i32, i32)>>, // (source, dest, timestamp)
    packet_set: RefCell<HashSet<(i32, i32, i32)>>, // For O(1) duplicate detection
    // For fast get_count: dest -> Vec<timestamp> (kept sorted for binary search)
    dest_timestamps: RefCell<HashMap<i32, Vec<i32>>>,
}

impl Router {
    fn new(memory_limit: i32) -> Self {
        Router {
            memory_limit,
            packets: RefCell::new(VecDeque::new()),
            packet_set: RefCell::new(HashSet::new()),
            dest_timestamps: RefCell::new(HashMap::new()),
        }
    }

    fn add_packet(&self, source: i32, destination: i32, timestamp: i32) -> bool {
        let packet_key = (source, destination, timestamp);

        // O(1) duplicate check
        if self.packet_set.borrow().contains(&packet_key) {
            return false;
        }

        // Add packet
        self.packets.borrow_mut().push_back(packet_key);
        self.packet_set.borrow_mut().insert(packet_key);

        // Update destination timestamps (insert in sorted order)
        {
            let mut dest_map = self.dest_timestamps.borrow_mut();
            let timestamps = dest_map.entry(destination).or_insert_with(Vec::new);

            // Binary search to find insertion position
            let pos = timestamps.binary_search(&timestamp).unwrap_or_else(|e| e);
            timestamps.insert(pos, timestamp);
        }

        // Handle memory limit - fix borrowing conflict
        let memory_exceeded = self.packets.borrow().len() > self.memory_limit as usize;
        if memory_exceeded {
            // Get the packet to remove first
            let packet_to_remove = self.packets.borrow_mut().pop_front();

            if let Some((src, dest, ts)) = packet_to_remove {
                self.packet_set.borrow_mut().remove(&(src, dest, ts));

                // Remove from destination timestamps
                let mut dest_map = self.dest_timestamps.borrow_mut();
                if let Some(timestamps) = dest_map.get_mut(&dest) {
                    if let Ok(pos) = timestamps.binary_search(&ts) {
                        timestamps.remove(pos);
                    }
                    if timestamps.is_empty() {
                        dest_map.remove(&dest);
                    }
                }
            }
        }

        true
    }

    fn forward_packet(&self) -> Vec<i32> {
        // Get the packet to remove first
        let packet_to_remove = self.packets.borrow_mut().pop_front();

        if let Some((source, dest, timestamp)) = packet_to_remove {
            // Remove from indices
            self.packet_set
                .borrow_mut()
                .remove(&(source, dest, timestamp));

            // Remove from destination timestamps
            {
                let mut dest_map = self.dest_timestamps.borrow_mut();
                if let Some(timestamps) = dest_map.get_mut(&dest) {
                    if let Ok(pos) = timestamps.binary_search(&timestamp) {
                        timestamps.remove(pos);
                    }
                    if timestamps.is_empty() {
                        dest_map.remove(&dest);
                    }
                }
            }

            vec![source, dest, timestamp]
        } else {
            vec![]
        }
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        let dest_map = self.dest_timestamps.borrow();

        if let Some(timestamps) = dest_map.get(&destination) {
            // Count timestamps in the inclusive range [start_time, end_time]
            timestamps
                .iter()
                .filter(|&&ts| ts >= start_time && ts <= end_time)
                .count() as i32
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::implement_router::Router;

    #[test]
    fn test_router_1() {
        let router = Router::new(3); // Initialize Router with memoryLimit of 3.
        assert_eq!(true, router.add_packet(1, 4, 90)); // Packet is added. Return True.
        assert_eq!(true, router.add_packet(2, 5, 90)); // Packet is added. Return True.
        assert_eq!(false, router.add_packet(1, 4, 90)); // This is a duplicate packet. Return False.
        assert_eq!(true, router.add_packet(3, 5, 95)); // Packet is added. Return True
        assert_eq!(true, router.add_packet(4, 5, 105)); // Packet is added, [1, 4, 90] is removed as number of packets exceeds memoryLimit. Return True.
        assert_eq!(vec![2, 5, 90], router.forward_packet()); // Return [2, 5, 90] and remove it from router.
        assert_eq!(true, router.add_packet(5, 2, 110)); // Packet is added. Return True.
        assert_eq!(1, router.get_count(5, 100, 110)); // The only packet with destination 5 and timestamp in the inclusive range [100, 110] is [4, 5, 105]. Return 1.
    }

    #[test]
    fn test_router_2() {
        let router = Router::new(3);
        assert_eq!(true, router.add_packet(7, 4, 90));
        assert_eq!(vec![7, 4, 90], router.forward_packet());
        assert_eq!(vec![0; 0], router.forward_packet());
    }

    #[test]
    fn test_router_3() {
        let router = Router::new(4);
        assert_eq!(true, router.add_packet(4, 2, 1));
        assert_eq!(true, router.add_packet(3, 2, 1));
        assert_eq!(2, router.get_count(2, 1, 1));
    }
}
