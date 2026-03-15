// 1622. Fancy Sequence
// https://leetcode.com/problems/fancy-sequence/

struct Fancy {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        todo!()
    }

    fn append(&self, val: i32) {
        todo!()
    }

    fn add_all(&self, inc: i32) {
        todo!()
    }

    fn mult_all(&self, m: i32) {
        todo!()
    }

    fn get_index(&self, idx: i32) -> i32 {
        todo!()
    }
}

/**
 * Your Fancy object will be instantiated and called as such:
 * let obj = Fancy::new();
 * obj.append(val);
 * obj.add_all(inc);
 * obj.mult_all(m);
 * let ret_4: i32 = obj.get_index(idx);
 */

#[cfg(test)]
mod tests {
    use crate::hard::fancy_sequence::Fancy;

    #[test]
    fn test_fancy_1() {
        // ["Fancy", "append", "addAll", "append", "multAll", "getIndex", "addAll", "append", "multAll", "getIndex", "getIndex", "getIndex"]
        // [[],      [2],      [3],      [7],      [2],       [0],        [3],      [10],     [2],       [0],         [1],       [2]]
        // [null,     null,    null,     null,     null,      10,         null,     null,      null,     26,          34,        20]
        let fancy = Fancy::new();
        fancy.append(2);
        fancy.add_all(3);
        fancy.append(7);
        fancy.mult_all(2);
        assert_eq!(10, fancy.get_index(0));
        fancy.add_all(3);
        fancy.append(10);
        fancy.mult_all(2);
        assert_eq!(26, fancy.get_index(0));
        assert_eq!(34, fancy.get_index(1));
        assert_eq!(20, fancy.get_index(2));
    }
}
