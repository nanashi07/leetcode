// 2353. Design a Food Rating System
// https://leetcode.com/problems/design-a-food-rating-system/

use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};

/**
 * Your FoodRatings object will be instantiated and called as such:
 * let obj = FoodRatings::new(foods, cuisines, ratings);
 * obj.change_rating(food, newRating);
 * let ret_2: String = obj.highest_rated(cuisine);
 */
struct FoodRatings {
    // Map from food name to its cuisine
    food_to_cuisine: HashMap<String, String>,
    // Map from food name to its current rating
    food_to_rating: RefCell<HashMap<String, i32>>,
    // Map from cuisine to a sorted set of (rating, food_name) pairs
    // BTreeSet keeps items sorted, with higher ratings first (due to Reverse)
    // and lexicographically smaller names first for same ratings
    cuisine_to_foods: RefCell<HashMap<String, BTreeSet<(std::cmp::Reverse<i32>, String)>>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_to_cuisine = HashMap::new();
        let mut food_to_rating = HashMap::new();
        let mut cuisine_to_foods: HashMap<String, BTreeSet<(std::cmp::Reverse<i32>, String)>> =
            HashMap::new();

        for i in 0..foods.len() {
            let food = &foods[i];
            let cuisine = &cuisines[i];
            let rating = ratings[i];

            food_to_cuisine.insert(food.clone(), cuisine.clone());
            food_to_rating.insert(food.clone(), rating);

            cuisine_to_foods
                .entry(cuisine.clone())
                .or_insert_with(BTreeSet::new)
                .insert((std::cmp::Reverse(rating), food.clone()));
        }

        Self {
            food_to_cuisine,
            food_to_rating: RefCell::new(food_to_rating),
            cuisine_to_foods: RefCell::new(cuisine_to_foods),
        }
    }

    fn change_rating(&self, food: String, new_rating: i32) {
        let cuisine = &self.food_to_cuisine[&food];
        let mut food_to_rating = self.food_to_rating.borrow_mut();
        let mut cuisine_to_foods = self.cuisine_to_foods.borrow_mut();

        // Get the old rating
        let old_rating = food_to_rating[&food];

        // Remove the old entry from the cuisine's BTreeSet
        let cuisine_set = cuisine_to_foods.get_mut(cuisine).unwrap();
        cuisine_set.remove(&(std::cmp::Reverse(old_rating), food.clone()));

        // Update the rating
        food_to_rating.insert(food.clone(), new_rating);

        // Add the new entry to the cuisine's BTreeSet
        cuisine_set.insert((std::cmp::Reverse(new_rating), food));
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let cuisine_to_foods = self.cuisine_to_foods.borrow();
        let foods_set = &cuisine_to_foods[&cuisine];

        // The first element in the BTreeSet is the highest rated
        // (due to Reverse ordering on rating and natural ordering on name)
        foods_set.iter().next().unwrap().1.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::medium::design_a_food_rating_system::FoodRatings;

    #[test]
    fn test_food_ratings_1() {
        let foods = ["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let cuisines = [
            "korean", "japanese", "japanese", "greek", "japanese", "korean",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
        let ratings = [9, 12, 8, 15, 14, 7].to_vec();
        let obj = FoodRatings::new(foods, cuisines, ratings);

        // Initially: kimchi(9), bulgogi(7) for korean -> kimchi is highest
        assert_eq!(obj.highest_rated("korean".to_string()), "kimchi");

        // Initially: miso(12), sushi(8), ramen(14) for japanese -> ramen is highest
        assert_eq!(obj.highest_rated("japanese".to_string()), "ramen");

        // Change sushi rating to 16
        obj.change_rating("sushi".to_string(), 16);
        // Now: miso(12), sushi(16), ramen(14) for japanese -> sushi is highest
        assert_eq!(obj.highest_rated("japanese".to_string()), "sushi");

        // Change ramen rating to 16 (same as sushi)
        obj.change_rating("ramen".to_string(), 16);
        // Now: miso(12), sushi(16), ramen(16) for japanese -> ramen comes first lexicographically
        assert_eq!(obj.highest_rated("japanese".to_string()), "ramen");
    }
}
