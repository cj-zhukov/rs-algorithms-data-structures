use std::collections::HashMap;

#[derive(Debug, Default)]
pub struct BankingSystemImpl {
    pub data: HashMap<String, (i32, i32)>, // acc_name, (timestamp, amount)
}


impl BankingSystemImpl {
    pub fn new() -> Self {
        Self::default()
    }
}

impl BankingSystemImpl {
    pub fn deposit(
        &mut self, 
        timestamp: i32, 
        account_id: String, 
        amount: i32,
    ) -> Option<i32> {
        match self.data.get_mut(&account_id) {
            Some(acc) => {
                acc.0 = timestamp;
                acc.1 += amount;
                Some(amount)
            },
            None => None,
        }
    }

    pub fn create_account(
        &mut self, 
        timestamp: i32, 
        account_id: String,
    ) -> bool {
        match self.data.get(&account_id) {
            Some(_) => return false,
            None => { 
                self.data.insert(account_id, (timestamp, 0));
                return true
            }
        }
    }

    pub fn transfer(
        &mut self,
        timestamp: i32,
        source_account_id: String,
        target_account_id: String,
        amount: i32,
    ) -> Option<i32> {
        // accounts must exist
        if !self.data.contains_key(&source_account_id)
            || !self.data.contains_key(&target_account_id)
        {
            return None;
        }

        // Borrow both entries mutably without violating borrow rules
        let [Some(src), Some(tar)] =
            self.data.get_disjoint_mut([&source_account_id, &target_account_id])
        else {
            // happens only if source == target
            return None;
        };

        // timestamp update
        src.0 = timestamp;
        tar.0 = timestamp;
        // balance check
        if src.1 < amount {
            return None;
        }

        // transfer
        src.1 -= amount;
        tar.1 += amount;

        Some(src.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bank_account() {
        let mut acc = BankingSystemImpl::new();
        acc.create_account(1, "acc1".to_string());
        acc.create_account(2, "acc2".to_string());

        acc.deposit(3, "acc1".to_string(), 1000);
        acc.deposit(4, "acc2".to_string(), 1000);
        let acc1 = acc.data.get("acc1");
        assert_eq!(acc1.unwrap().1, 1000);
        let acc2 = acc.data.get("acc2");
        assert_eq!(acc2.unwrap().1, 1000);

        acc.transfer(42, "acc1".to_string(), "acc2".to_string(), 500);
        let acc1 = acc.data.get("acc1");
        assert_eq!(acc1.unwrap().1, 500);
        let acc2 = acc.data.get("acc2");
        assert_eq!(acc2.unwrap().1, 1500);
    }

}