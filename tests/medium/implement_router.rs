// # 3508. Implement Router
// https://leetcode.com/problems/implement-router/

/**
 * Your Router object will be instantiated and called as such:
 * let obj = Router::new(memoryLimit);
 * let ret_1: bool = obj.add_packet(source, destination, timestamp);
 * let ret_2: Vec<i32> = obj.forward_packet();
 * let ret_3: i32 = obj.get_count(destination, startTime, endTime);
 */
struct Router {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Router {
    fn new(memoryLimit: i32) -> Self {
        todo!()
    }

    fn add_packet(&self, source: i32, destination: i32, timestamp: i32) -> bool {
        todo!()
    }

    fn forward_packet(&self) -> Vec<i32> {
        todo!()
    }

    fn get_count(&self, destination: i32, start_time: i32, end_time: i32) -> i32 {
        todo!()
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
}
