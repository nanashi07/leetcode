// 1622. Fancy Sequence
// https://leetcode.com/problems/fancy-sequence/

use std::cell::RefCell;

const MOD: i64 = 1_000_000_007;

struct Fancy {
    state: RefCell<State>,
}

struct State {
    collapsed_len: usize,
    collapsed_value: i64,
    suffix_values: Vec<i64>,
    suffix_mul: i64,
    suffix_add: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Fancy {
    fn new() -> Self {
        Self {
            state: RefCell::new(State {
                collapsed_len: 0,
                collapsed_value: 0,
                suffix_values: Vec::new(),
                suffix_mul: 1,
                suffix_add: 0,
            }),
        }
    }

    fn append(&self, val: i32) {
        let mut state = self.state.borrow_mut();
        let normalized = normalize(val as i64, state.suffix_mul, state.suffix_add);
        state.suffix_values.push(normalized);
    }

    fn add_all(&self, inc: i32) {
        let mut state = self.state.borrow_mut();
        state.collapsed_value = (state.collapsed_value + inc as i64).rem_euclid(MOD);
        state.suffix_add = (state.suffix_add + inc as i64).rem_euclid(MOD);
    }

    fn mult_all(&self, m: i32) {
        let mut state = self.state.borrow_mut();
        let multiplier = (m as i64).rem_euclid(MOD);

        if multiplier == 0 {
            state.collapsed_len += state.suffix_values.len();
            state.collapsed_value = 0;
            state.suffix_values.clear();
            state.suffix_mul = 1;
            state.suffix_add = 0;
            return;
        }

        state.collapsed_value = state.collapsed_value * multiplier % MOD;
        state.suffix_mul = state.suffix_mul * multiplier % MOD;
        state.suffix_add = state.suffix_add * multiplier % MOD;
    }

    fn get_index(&self, idx: i32) -> i32 {
        if idx < 0 {
            return -1;
        }

        let state = self.state.borrow();
        let index = idx as usize;
        let len = state.collapsed_len + state.suffix_values.len();
        if index >= len {
            return -1;
        }

        if index < state.collapsed_len {
            return state.collapsed_value as i32;
        }

        let normalized = state.suffix_values[index - state.collapsed_len];
        ((normalized * state.suffix_mul + state.suffix_add) % MOD) as i32
    }
}

fn normalize(value: i64, mul: i64, add: i64) -> i64 {
    (value - add).rem_euclid(MOD) * mod_pow(mul, MOD - 2) % MOD
}

fn mod_pow(mut base: i64, mut exp: i64) -> i64 {
    let mut result = 1_i64;
    while exp > 0 {
        if exp & 1 == 1 {
            result = result * base % MOD;
        }
        base = base * base % MOD;
        exp >>= 1;
    }
    result
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
