// 1352. Product of the Last K Numbers
// https://leetcode.com/problems/product-of-the-last-k-numbers/

struct ProductOfNumbers {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl ProductOfNumbers {
    fn new() -> Self {
        todo!()
    }

    fn add(&self, num: i32) {
        todo!()
    }

    fn get_product(&self, k: i32) -> i32 {
        todo!()
    }
}

/**
 * Your ProductOfNumbers object will be instantiated and called as such:
 * let obj = ProductOfNumbers::new();
 * obj.add(num);
 * let ret_2: i32 = obj.get_product(k);
 */

#[cfg(test)]
mod tests {
    use crate::medium::product_of_the_last_k_numbers::ProductOfNumbers;

    #[test]
    fn test_product_of_numbers_1() {
        // ["ProductOfNumbers","add","add","add","add","add","getProduct","getProduct","getProduct","add","getProduct"]
        // [[],                [3],  [0],  [2],  [5],  [4],  [2],         [3],         [4],         [8],   [2]]
        // [null,              null, null, null, null, null, 20,          40,          0,           null,   32]

        // Explanation
        // ProductOfNumbers productOfNumbers = new ProductOfNumbers();
        // productOfNumbers.add(3);        // [3]
        // productOfNumbers.add(0);        // [3,0]
        // productOfNumbers.add(2);        // [3,0,2]
        // productOfNumbers.add(5);        // [3,0,2,5]
        // productOfNumbers.add(4);        // [3,0,2,5,4]
        // productOfNumbers.getProduct(2); // return 20. The product of the last 2 numbers is 5 * 4 = 20
        // productOfNumbers.getProduct(3); // return 40. The product of the last 3 numbers is 2 * 5 * 4 = 40
        // productOfNumbers.getProduct(4); // return 0. The product of the last 4 numbers is 0 * 2 * 5 * 4 = 0
        // productOfNumbers.add(8);        // [3,0,2,5,4,8]
        // productOfNumbers.getProduct(2); // return 32. The product of the last 2 numbers is 4 * 8 = 32

        let numbers = ProductOfNumbers::new();
        todo!()
    }
}
