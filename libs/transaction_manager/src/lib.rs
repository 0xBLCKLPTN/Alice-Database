/*        Transaction Manager.
    Transaction Manager is a simple vector
   with FIFO (FIRST IN, FIRST OUT) methodology.
   FIFO an asset and inventory management approach
   in which assets produced or acquired first are sold,
   used, or disposed of first.
    

  Author - Daniil (nullxjanus) Ermolaev.
*/

#[derive(Debug)]
pub struct Transaction {
    pub uid: String,
    pub query: String,
    pub username: String,
}

pub struct TransactionManager {
    pub list_of_transactions: Vec<Transaction>,
}

impl TransactionManager {
    pub fn init() -> Self {
        let mut list_of_transactions: Vec<Transaction> = Vec::new();
        Self { list_of_transactions }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let mut list_of_transactions: Vec<Transaction> = Vec::with_capacity(capacity);
        Self { list_of_transactions }
    } 

    pub fn get_list(&self) -> &Vec<Transaction> {
        &self.list_of_transactions
    }

    pub fn add(&mut self, transaction: Transaction) {
        self.list_of_transactions.push(transaction);
    }

    pub fn remove_first(&mut self) {
        if !self.list_of_transactions.is_empty() {
            self.list_of_transactions.remove(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let mut tm = TransactionManager::init();
        tm.add(Transaction {
            uid: "uid".to_string(),
            query: "query".to_string(),
            username: "username".to_string()
        } );
        tm.get_list();
        tm.remove_first();
    }
}
