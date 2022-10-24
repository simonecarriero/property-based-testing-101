#[derive(Clone, Debug)]
struct Wallet {
    quantity: i16,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet { quantity: 0 }
    }

    pub fn execute(&mut self, operation: &Operation) -> Result<i16, anyhow::Error> {
        for transaction in &operation.0 {
            match transaction {
                Transaction::Buy { quantity } => self.buy(*quantity)?,
                Transaction::Sell { quantity } => self.sell(*quantity)?,
            }
        }
        Ok(self.quantity)
    }

    fn buy(&mut self, quantity: u16) -> Result<(), anyhow::Error> {
        self.quantity += quantity as i16;
        Ok(())
    }

    fn sell(&mut self, quantity: u16) -> Result<(), anyhow::Error> {
        self.quantity -= quantity as i16;
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Operation(Vec<Transaction>);

#[derive(Clone, Debug)]
pub enum Transaction {
    Buy { quantity: u16 },
    Sell { quantity: u16 },
}

#[cfg(test)]
mod tests {
    use crate::wallet::{Operation, Transaction, Wallet};

    #[test]
    fn buy_some_stock() {
        let mut wallet = Wallet::new();
        let operation = Operation(vec![Transaction::Buy { quantity: 7 }]);

        let result = wallet.execute(&operation);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn buy_and_sell_some_stock() {
        let mut wallet = Wallet::new();
        let operation = Operation(vec![
            Transaction::Buy { quantity: 7 },
            Transaction::Sell { quantity: 2 },
        ]);

        let result = wallet.execute(&operation);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 5);
    }
}
