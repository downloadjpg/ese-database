/*
In-memory key-value database with transaction support. Written in Rust by Callie Foster for ESE Assignment 13.
*/

use std::result::Result;
pub struct Database {
    store: std::collections::HashMap<String, i32>,
    current_transaction: Option<Transaction>,
}

struct Transaction {
    put_pairs: Vec<(String, i32)>,
}

pub enum DatabaseError {
    NoTransactionInProgress,
    TransactionAlreadyInProgress,
}

impl Database {
    pub fn new() -> Self {
        Database {
            store: std::collections::HashMap::new(),
            current_transaction: None,
        }
    }
    pub fn begin_transaction(&mut self) -> Result<(), DatabaseError> {
        match &self.current_transaction {
            Some(_) => Err(DatabaseError::TransactionAlreadyInProgress),
            None => {
                self.current_transaction = Some(Transaction { put_pairs: vec![] });
                Ok(())
            }
        }
    }

    pub fn put(&mut self, key: &str, value: i32) -> Result<(), DatabaseError> {
        match &mut self.current_transaction {
            Some(tx) => {
                tx.put_pairs.push((key.to_owned(), value));
                Ok(())
            }
            None => Err(DatabaseError::NoTransactionInProgress),
        }
    }
    pub fn get(&self, key: &str) -> Option<i32> {
        self.store.get(key).cloned()
    }
    pub fn commit(&mut self) -> Result<(), DatabaseError> {
        let pairs = self.current_transaction
            .take()
            .ok_or(DatabaseError::NoTransactionInProgress)?
            .put_pairs;

        for (key, value) in pairs {
            self.store.insert(key, value);
        }
        Ok(())
    }

    pub fn rollback(&mut self) -> Result<(), DatabaseError> {
        match self.current_transaction.take() {
            Some(_) => Ok(()),
            None => Err(DatabaseError::NoTransactionInProgress),
        }
    }
}


// Tests:
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_database() {
        let mut db = Database::new();
        // Test commands that should fail when no transaction is in progress
        assert!(db.get("A").is_none());
        assert!(db.put("A", 5).is_err());
        assert!(db.commit().is_err());
        assert!(db.rollback().is_err());

        assert!(db.begin_transaction().is_ok());
        // Put A = 5 in transaction
        assert!(db.put("A", 5).is_ok());
        // Get A should still be None, since transaction not committed
        assert!(db.get("A").is_none());
        // Add another value
        assert!(db.put("B", 20).is_ok());
        // Change A to 6
        assert!(db.put("A", 6).is_ok());
        // Commit transaction
        assert!(db.commit().is_ok());
        // Check values
        assert_eq!(db.get("A"), Some(6));
        assert_eq!(db.get("B"), Some(20));
    }
}
/*
Instructipons:

1. In the language of your choice you have to build an in-memory key-value database that
supports the following functions (See Fig 1 for a sample Java interface definition)
    a. begin_transaction()
    b. put(key, value)
    c. get(key)
    d. commit()
    e. rollback()
2. Keys should be string and values should be integers.
3. put(key, val) will create a new key with the provided value if a key doesn’t exist.
Otherwise it will update the value of an existing key.
4. get(key) will return the value associated with the key or null if the key doesn’t exist.
5. If put(key, val) is called when a transaction is not in progress throw an exception
6. get(key) can be called anytime even when a transaction is not in progress
7. begin_transaction() starts a new transaction.
8. At a time only a single transaction may exist.
9. Within a transaction you can make as many changes to as many keys as you like.
However, they should not be “visible” to get(), until the transaction is committed.
10. A transaction ends when either commit() or rollback() is called
11. commit() applies changes made within the transaction to the main state. Allowing any
future gets() to “see” the changes made within the transaction
12. rollback() should abort all the changes made within the transaction and everything
should go back to the way it was before.
 */