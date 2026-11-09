// 1606. Find Servers That Handled Most Number of Requests
// https://leetcode.com/problems/find-servers-that-handled-most-number-of-requests/

struct Solution;

impl Solution {
    pub fn busiest_servers(k: i32, arrival: Vec<i32>, load: Vec<i32>) -> Vec<i32> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::find_servers_that_handled_most_number_of_requests::Solution;

    #[test]
    fn test_busiest_servers_1() {
        let k = 3;
        let arrival = [1, 2, 3, 4, 5].to_vec();
        let load = [5, 2, 3, 3, 3].to_vec();
        assert_eq!([1].to_vec(), Solution::busiest_servers(k, arrival, load));
    }

    #[test]
    fn test_busiest_servers_2() {
        let k = 3;
        let arrival = [1, 2, 3, 4].to_vec();
        let load = [1, 2, 1, 2].to_vec();
        assert_eq!([0].to_vec(), Solution::busiest_servers(k, arrival, load));
    }

    #[test]
    fn test_busiest_servers_3() {
        let k = 3;
        let arrival = [1, 2, 3].to_vec();
        let load = [10, 12, 11].to_vec();
        assert_eq!(
            [0, 1, 2].to_vec(),
            Solution::busiest_servers(k, arrival, load)
        );
    }
}
