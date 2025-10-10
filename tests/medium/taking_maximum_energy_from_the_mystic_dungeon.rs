// # 3147. Taking Maximum Energy From the Mystic Dungeon
// https://leetcode.com/problems/taking-maximum-energy-from-the-mystic-dungeon/

struct Solution;

impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        println!("energy: {:?}, k: {k}", &energy);

        let mut max = i32::MIN;
        let k = k as usize;

        for i in 0..k {
            let len = 1 + energy.len() / k;
            println!("{} / {} = {}", &energy.len(), k, len);
            let mut values = vec![0; 0];
            for j in (0..len).rev() {
                let p = i + j * k;
                if p < energy.len() {
                    values.insert(0, energy[p]);
                    if values.len() > 1 {
                        values[0] += values[1];
                    }
                    println!("p: {p}, v: {:?}, values: {:?}", &energy[p], &values);
                }
            }
            max = max.max(*values.iter().max().unwrap());
            println!("max: {}", &max);
        }

        max
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::taking_maximum_energy_from_the_mystic_dungeon::Solution;

    #[test]
    fn test_maximum_energy_1() {
        let energy = [5, 2, -10, -5, 1].to_vec();
        let k = 3;
        assert_eq!(3, Solution::maximum_energy(energy, k));
    }

    #[test]
    fn test_maximum_energy_2() {
        let energy = [-2, -3, -1].to_vec();
        let k = 2;
        assert_eq!(-1, Solution::maximum_energy(energy, k));
    }
}
