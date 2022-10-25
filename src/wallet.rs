use anyhow::anyhow;

#[derive(Clone, Debug)]
struct Wallet {
    quantity: Quantity,
}

impl Wallet {
    pub fn new() -> Wallet {
        Wallet { quantity: 0 }
    }

    pub fn execute(&mut self, operation: &Operation) -> Result<Quantity, anyhow::Error> {
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
        self.quantity += quantity as Quantity;
        Ok(())
    }

    fn sell(&mut self, quantity: u16) -> Result<(), anyhow::Error> {
        if quantity > self.quantity as u16 {
            return Err(anyhow!("Not enough stock to sell"));
        }
        self.quantity -= quantity as Quantity;
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

type Quantity = i16;

#[cfg(test)]
mod tests {
    use crate::wallet::{Operation, Transaction, Wallet};
    use quickcheck::QuickCheck;
    use quickcheck::{Gen, StdThreadGen};

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

    #[test]
    fn property_wallet_quantity_is_never_negative() {
        fn property(operation: Operation) -> bool {
            let mut wallet = Wallet::new();
            wallet.execute(&operation).ok();
            wallet.quantity >= 0
        }

        quickcheck().quickcheck(property as fn(Operation) -> bool);
    }

    fn quickcheck() -> QuickCheck<StdThreadGen> {
        QuickCheck::new().gen(StdThreadGen::new(u16::max_value() as usize))
    }

    impl quickcheck::Arbitrary for Transaction {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let quantity = u16::arbitrary(g);
            match bool::arbitrary(g) {
                true => Transaction::Buy { quantity },
                false => Transaction::Sell {
                    quantity: quantity / 100,
                },
            }
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            match self {
                Transaction::Buy { quantity } => {
                    Box::new(quantity.shrink().map(|q| Transaction::Buy { quantity: q }))
                }
                Transaction::Sell { quantity } => {
                    Box::new(quantity.shrink().map(|q| Transaction::Sell { quantity: q }))
                }
            }
        }
    }

    impl quickcheck::Arbitrary for Operation {
        fn arbitrary<G: Gen>(g: &mut G) -> Self {
            let mut transactions = vec![];
            let number_of_transactions = u16::arbitrary(g) / 1000;
            let mut count = 0;

            while count < number_of_transactions {
                transactions.push(Transaction::arbitrary(g));
                count += 1;
            }

            Operation(transactions)
        }
        fn shrink(&self) -> Box<dyn Iterator<Item = Self>> {
            Box::new(self.0.shrink().map(|x| Operation(x)))
        }
    }
}
