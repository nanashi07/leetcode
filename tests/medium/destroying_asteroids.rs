// 2126. Destroying Asteroids
// https://leetcode.com/problems/destroying-asteroids/

struct Solution;

impl Solution {
    pub fn asteroids_destroyed(mass: i32, asteroids: Vec<i32>) -> bool {
        let mut current_mass = i64::from(mass);
        let mut asteroids = asteroids;
        asteroids.sort_unstable();

        for asteroid in asteroids {
            let asteroid = i64::from(asteroid);
            if current_mass < asteroid {
                return false;
            }
            current_mass += asteroid;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::destroying_asteroids::Solution;

    #[test]
    fn test_asteroids_destroyed_1() {
        let mass = 10;
        let asteroids = [3, 9, 19, 5, 21].to_vec();
        assert_eq!(true, Solution::asteroids_destroyed(mass, asteroids));
    }

    #[test]
    fn test_asteroids_destroyed_2() {
        let mass = 5;
        let asteroids = [4, 9, 23, 4].to_vec();
        assert_eq!(false, Solution::asteroids_destroyed(mass, asteroids));
    }
}
