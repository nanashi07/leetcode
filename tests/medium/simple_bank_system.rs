// # 2043. Simple Bank System
// https://leetcode.com/problems/simple-bank-system/

struct Bank {}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Bank {
    fn new(balance: Vec<i64>) -> Self {
        todo!()
    }

    fn transfer(&self, account1: i32, account2: i32, money: i64) -> bool {
        todo!()
    }

    fn deposit(&self, account: i32, money: i64) -> bool {
        todo!()
    }

    fn withdraw(&self, account: i32, money: i64) -> bool {
        todo!()
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
