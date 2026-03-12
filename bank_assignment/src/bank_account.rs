#[derive(Debug)]
pub struct BankAccount {
    balance: f64,
}

impl BankAccount {
    pub fn new(initial_balance: f64) -> BankAccount {
        // Implement this method
        BankAccount {
            balance: if initial_balance >= 0.0 {initial_balance} else {0.0},
        }
    }

    pub fn deposit(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 {
            self.balance += amount;
        }
    }

    pub fn withdraw(&mut self, amount: f64) {
        // Implement this method
        if amount > 0.0 && amount <= self.balance {
            self.balance -= amount;
        }
    }

    pub fn balance(&self) -> f64 {
        // Implement this method
        self.balance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 1e-10;

    #[test]
    fn test_new_account() {
        // Write a test for creating a new account
        let acc = BankAccount::new(100.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_new_account_negative_balance() {
        // Write a test for creating a new account
        let acc = BankAccount::new(-50.0);
        assert!((acc.balance() - 0.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit() {
        // Write a test for depositing money
        let mut acc = BankAccount::new(100.0);
        acc.deposit(50.0);
        assert!((acc.balance() - 150.0).abs() < EPSILON);
    }

    #[test]
    fn test_deposit_negative_amount() {
        // Write a test for depositing money
        let mut acc = BankAccount::new(100.0);
        acc.deposit(-20.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw() {
        // Write a test for withdrawing money
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(40.0);
        assert!((acc.balance() - 60.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw_more_than_balance() {
        // Write a test for withdrawing money
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(200.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }

    #[test]
    fn test_withdraw_negative_amount() {
        // Write a test for withdrawing money
        let mut acc = BankAccount::new(100.0);
        acc.withdraw(-10.0);
        assert!((acc.balance() - 100.0).abs() < EPSILON);
    }
}