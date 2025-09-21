// # 1912. Design Movie Rental System
// https://leetcode.com/problems/design-movie-rental-system/

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */
struct MovieRentingSystem {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(n: i32, entries: Vec<Vec<i32>>) -> Self {
        todo!()
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        todo!()
    }

    fn rent(&self, shop: i32, movie: i32) {
        todo!()
    }

    fn drop(&self, shop: i32, movie: i32) {
        todo!()
    }

    fn report(&self) -> Vec<Vec<i32>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::design_movie_rental_system::MovieRentingSystem;

    #[test]
    fn test_movie_renting_system_1() {
        let movie_renting_system = MovieRentingSystem::new(
            3,
            vec![
                vec![0, 1, 5],
                vec![0, 2, 6],
                vec![0, 3, 7],
                vec![1, 1, 4],
                vec![1, 2, 7],
                vec![2, 1, 5],
            ],
        );
        assert_eq!(vec![1, 0, 2], movie_renting_system.search(1)); // return [1, 0, 2], Movies of ID 1 are unrented at shops 1, 0, and 2. Shop 1 is cheapest; shop 0 and 2 are the same price, so order by shop number.
        movie_renting_system.rent(0, 1); // Rent movie 1 from shop 0. Unrented movies at shop 0 are now [2,3].
        movie_renting_system.rent(1, 2); // Rent movie 2 from shop 1. Unrented movies at shop 1 are now [1].
        assert_eq!(vec![vec![0, 1], vec![1, 2]], movie_renting_system.report()); // return [[0, 1], [1, 2]]. Movie 1 from shop 0 is cheapest, followed by movie 2 from shop 1.
        movie_renting_system.drop(1, 2); // Drop off movie 2 at shop 1. Unrented movies at shop 1 are now [1,2].
        assert_eq!(vec![0, 1], movie_renting_system.search(2)); // return [0, 1]. Movies of ID 2 are unrented at shops 0 and 1. Shop 0 is cheapest, followed by shop 1.
    }
}
