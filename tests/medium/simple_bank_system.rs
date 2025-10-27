// # 2043. Simple Bank System
// https://leetcode.com/problems/simple-bank-system/

use std::cell::RefCell;

struct Bank {
    balance: RefCell<Vec<i64>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        Bank {
            balance: RefCell::new(balance),
        }
    }

    fn transfer(&self, account1: i32, account2: i32, money: i64) -> bool {
        let mut balances = self.balance.borrow_mut();
        let n = balances.len() as i32;

        // Validate accounts exist (1-indexed)
        if account1 < 1 || account1 > n || account2 < 1 || account2 > n {
            return false;
        }

        let idx1 = (account1 - 1) as usize;
        let idx2 = (account2 - 1) as usize;

        // Check if account1 has sufficient balance
        if balances[idx1] < money {
            return false;
        }

        // Perform transfer
        balances[idx1] -= money;
        balances[idx2] += money;
        true
    }

    fn deposit(&self, account: i32, money: i64) -> bool {
        let mut balances = self.balance.borrow_mut();
        let n = balances.len() as i32;

        // Validate account exists (1-indexed)
        if account < 1 || account > n {
            return false;
        }

        let idx = (account - 1) as usize;
        balances[idx] += money;
        true
    }

    fn withdraw(&self, account: i32, money: i64) -> bool {
        let mut balances = self.balance.borrow_mut();
        let n = balances.len() as i32;

        // Validate account exists (1-indexed)
        if account < 1 || account > n {
            return false;
        }

        let idx = (account - 1) as usize;

        // Check if account has sufficient balance
        if balances[idx] < money {
            return false;
        }

        balances[idx] -= money;
        true
    }
}

/**
 * Your Bank object will be instantiated and called as such:
 * let obj = Bank::new(balance);
 * let ret_1: bool = obj.transfer(account1, account2, money);
 * let ret_2: bool = obj.deposit(account, money);
 * let ret_3: bool = obj.withdraw(account, money);
 */

#[cfg(test)]
mod tests {
    use crate::medium::simple_bank_system::Bank;

    #[test]
    fn test_bank_1() {
        let bank = Bank::new([10, 100, 20, 50, 30].to_vec());
        assert_eq!(true, bank.withdraw(3, 10));
        assert_eq!(true, bank.transfer(5, 1, 20));
        assert_eq!(true, bank.deposit(5, 20));
        assert_eq!(false, bank.transfer(3, 4, 15));
        assert_eq!(false, bank.withdraw(10, 50));
    }
}
