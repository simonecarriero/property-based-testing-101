use anyhow::anyhow;

#[derive(Clone, Debug)]
struct Wallet {
    quantity: i16,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet { quantity: 0 }
    }

    pub fn execute(&mut self, operation: &Operation) -> Result<i16, anyhow::Error> {
        let initial_quantity = self.quantity;

        let result = (|| {
            for transaction in &operation.0 {
                match transaction {
                    Transaction::Buy { quantity } => self.buy(*quantity)?,
                    Transaction::Sell { quantity } => self.sell(*quantity)?,
                }
            }
            Ok(())
        })();

        if let Err(e) = result {
            self.quantity = initial_quantity;
            return Err(e);
        }

        Ok(self.quantity)
    }

    fn buy(&mut self, quantity: u16) -> Result<(), anyhow::Error> {
        self.quantity += quantity as i16;
        Ok(())
    }

    fn sell(&mut self, quantity: u16) -> Result<(), anyhow::Error> {
        if quantity > self.quantity as u16 {
            return Err(anyhow!("Not enough stock to sell"));
        }
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

    #[test]
    fn fail_when_selling_more_than_owned() {
        let mut wallet = Wallet::new();
        let operation = Operation(vec![
            Transaction::Buy { quantity: 5 },
            Transaction::Sell { quantity: 7 },
        ]);

        let result = wallet.execute(&operation);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Not enough stock to sell");
        assert_eq!(wallet.quantity, 0);
    }
}
