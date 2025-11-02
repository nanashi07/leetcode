// 1912. Design Movie Rental System
// https://leetcode.com/problems/design-movie-rental-system/
use std::collections::{BTreeSet, HashMap};

/**
 * Your MovieRentingSystem object will be instantiated and called as such:
 * let obj = MovieRentingSystem::new(n, entries);
 * let ret_1: Vec<i32> = obj.search(movie);
 * obj.rent(shop, movie);
 * obj.drop(shop, movie);
 * let ret_4: Vec<Vec<i32>> = obj.report();
 */
struct MovieRentingSystem {
    // For each movie, maintain a sorted set of (price, shop_id) for unrented copies
    unrented: HashMap<i32, BTreeSet<(i32, i32)>>,
    // Maintain a sorted set of (price, shop_id, movie_id) for rented movies
    rented: BTreeSet<(i32, i32, i32)>,
    // Store the price for each (shop, movie) pair
    prices: HashMap<(i32, i32), i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let mut unrented: HashMap<i32, BTreeSet<(i32, i32)>> = HashMap::new();
        let mut prices: HashMap<(i32, i32), i32> = HashMap::new();

        // Initialize the data structures with the given entries
        for entry in entries {
            let shop = entry[0];
            let movie = entry[1];
            let price = entry[2];

            // Store price for this shop-movie pair
            prices.insert((shop, movie), price);

            // Add to unrented movies (sorted by price, then by shop)
            unrented
                .entry(movie)
                .or_insert_with(BTreeSet::new)
                .insert((price, shop));
        }

        MovieRentingSystem {
            unrented,
            rented: BTreeSet::new(),
            prices,
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        // Return up to 5 cheapest unrented copies of the movie
        if let Some(shops) = self.unrented.get(&movie) {
            shops.iter().take(5).map(|(_, shop)| *shop).collect()
        } else {
            vec![]
        }
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = self.prices[&(shop, movie)];

        // Remove from unrented
        if let Some(shops) = self.unrented.get_mut(&movie) {
            shops.remove(&(price, shop));
            if shops.is_empty() {
                self.unrented.remove(&movie);
            }
        }

        // Add to rented
        self.rented.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = self.prices[&(shop, movie)];

        // Remove from rented
        self.rented.remove(&(price, shop, movie));

        // Add back to unrented
        self.unrented
            .entry(movie)
            .or_insert_with(BTreeSet::new)
            .insert((price, shop));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        // Return up to 5 cheapest rented movies
        self.rented
            .iter()
            .take(5)
            .map(|(_, shop, movie)| vec![*shop, *movie])
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::hard::design_movie_rental_system::MovieRentingSystem;

    #[test]
    fn test_movie_renting_system_1() {
        let mut movie_renting_system = MovieRentingSystem::new(
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
