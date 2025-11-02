// 1792. Maximum Average Pass Ratio
// https://leetcode.com/problems/maximum-average-pass-ratio/

struct Solution;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Debug)]
struct ClassGain {
    gain: f64,
    pass: i32,
    total: i32,
}

impl ClassGain {
    fn new(pass: i32, total: i32) -> Self {
        let gain = (total - pass) as f64 / (total as f64 * (total + 1) as f64);
        ClassGain { gain, pass, total }
    }

    fn add_student(&mut self) {
        self.pass += 1;
        self.total += 1;
        self.gain = (self.total - self.pass) as f64 / (self.total as f64 * (self.total + 1) as f64);
    }
}

impl PartialEq for ClassGain {
    fn eq(&self, other: &Self) -> bool {
        self.gain == other.gain
    }
}

impl Eq for ClassGain {}

impl PartialOrd for ClassGain {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ClassGain {
    fn cmp(&self, other: &Self) -> Ordering {
        // For max heap, we want higher gains to have higher priority
        self.gain
            .partial_cmp(&other.gain)
            .unwrap_or(Ordering::Equal)
    }
}

impl Solution {
    pub fn max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        let mut heap = BinaryHeap::new();

        // Initialize heap with all classes
        for class in classes.iter() {
            let pass = class[0];
            let total = class[1];
            heap.push(ClassGain::new(pass, total));
        }

        // Distribute extra students
        for _ in 0..extra_students {
            if let Some(mut best_class) = heap.pop() {
                best_class.add_student();
                heap.push(best_class);
            }
        }

        // Calculate final average
        let mut total_ratio = 0.0;
        let n = heap.len() as f64;

        while let Some(class) = heap.pop() {
            total_ratio += class.pass as f64 / class.total as f64;
        }

        total_ratio / n
    }

    // Time Limit Exceeded
    pub fn _max_average_ratio(classes: Vec<Vec<i32>>, extra_students: i32) -> f64 {
        println!(
            "classes: {:?}, extra_students: {}",
            &classes, &extra_students
        );

        let mut ext = vec![0; classes.iter().len()];
        let mut data = vec![vec![0.0; classes.len()]; extra_students as usize + 1];
        let mut extra_students = extra_students;

        for e in 0..=extra_students {
            for i in 0..classes.len() {
                data[e as usize][i] = (classes[i][0] + e) as f64 / (classes[i][1] + e) as f64;
                // if e > 0 {
                //     for xe in 0..e {
                //         data[e as usize][i] -= data[xe as usize][i];
                //     }
                // }
            }
        }

        let mut result: f64 = data[0].iter().sum();
        data.iter().for_each(|r| println!("{:?}", &r));

        while extra_students > 0 {
            let mut next = 0;
            let mut max = 0.0;
            for i in 0..data[0].len() {
                let inc = data[ext[i] + 1][i] - data[ext[i]][i];
                if inc > max {
                    max = inc;
                    next = i;
                }
            }
            ext[next] += 1;
            result += max;
            extra_students -= 1;
        }

        result / classes.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::maximum_average_pass_ratio::Solution;

    #[test]
    fn test_max_average_ratio_1() {
        let classes = [[1, 2], [3, 5], [2, 2]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let extra_students = 2;
        assert_eq!(
            0.78333,
            (Solution::max_average_ratio(classes, extra_students) * 100000.0).round() / 100000.0
        );
    }

    #[test]
    fn test_max_average_ratio_2() {
        let classes = [[2, 4], [3, 9], [4, 5], [2, 10]]
            .into_iter()
            .map(|l| l.to_vec())
            .collect::<Vec<Vec<i32>>>();
        let extra_students = 4;
        assert_eq!(
            0.53485,
            (Solution::max_average_ratio(classes, extra_students) * 100000.0).round() / 100000.0
        );
    }

    #[test]
    fn test_max_average_ratio_3() {
        let classes = [
            [222, 993],
            [433, 744],
            [30, 541],
            [228, 783],
            [449, 962],
            [508, 567],
            [239, 354],
            [237, 694],
            [225, 780],
            [471, 976],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<i32>>>();
        let extra_students = 7;
        assert_eq!(
            0.43146,
            (Solution::max_average_ratio(classes, extra_students) * 100000.0).round() / 100000.0
        );
    }

    #[test]
    fn test_max_average_ratio_4() {
        let classes = [
            [684, 883],
            [254, 259],
            [66, 797],
            [699, 987],
            [458, 828],
            [441, 563],
            [257, 555],
            [450, 872],
            [465, 551],
            [12, 406],
            [347, 857],
            [176, 265],
            [25, 498],
            [662, 813],
            [427, 956],
            [585, 1000],
            [20, 64],
            [364, 709],
            [142, 594],
            [129, 608],
            [142, 266],
            [284, 849],
            [408, 578],
            [177, 411],
            [92, 628],
            [240, 498],
            [8, 182],
            [325, 542],
            [513, 915],
            [665, 943],
            [449, 953],
            [655, 703],
            [232, 749],
            [245, 321],
            [507, 704],
            [491, 980],
            [231, 730],
            [346, 423],
            [574, 626],
            [746, 929],
            [670, 940],
            [282, 996],
            [225, 662],
            [50, 944],
            [74, 782],
            [524, 661],
            [378, 899],
            [164, 524],
            [785, 812],
            [209, 905],
            [306, 320],
            [307, 710],
            [566, 870],
            [170, 381],
            [719, 719],
            [476, 755],
            [88, 609],
            [127, 877],
            [621, 919],
            [527, 984],
            [387, 585],
            [160, 181],
            [257, 437],
            [223, 965],
            [584, 737],
            [776, 802],
            [54, 507],
            [404, 698],
            [653, 735],
            [357, 394],
            [528, 866],
            [169, 558],
            [42, 748],
            [93, 537],
            [262, 828],
            [104, 644],
            [274, 755],
            [86, 935],
            [983, 999],
            [143, 993],
            [632, 795],
            [863, 991],
            [676, 704],
            [84, 718],
            [456, 872],
            [247, 947],
            [872, 995],
            [392, 963],
            [822, 926],
            [407, 444],
            [169, 932],
            [334, 449],
            [130, 638],
            [500, 931],
            [218, 983],
        ]
        .into_iter()
        .map(|l| l.to_vec())
        .collect::<Vec<Vec<i32>>>();
        let extra_students = 5976;
        assert_eq!(
            0.58664,
            (Solution::max_average_ratio(classes, extra_students) * 100000.0).round() / 100000.0
        );
    }
}
