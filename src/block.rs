use crate::block::pb::TransactionStatus::TransactionstatusExecuted;
use crate::pb;

impl pb::Block {
    /// returns all transaction traces from the block.
    pub fn all_transaction_traces(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        if self.filtering_applied {
            return self.filtered_transaction_traces.iter();
        }
        self.unfiltered_transaction_traces.iter()
    }

    /// returns all transaction traces which have the status `executed`
    pub fn executed_transaction_traces(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        return self
            .all_transaction_traces()
            .filter(|trx| trx.receipt.as_ref().unwrap().status == TransactionstatusExecuted as i32);
    }

    /// returns the number of transaction traces included in this block
    pub fn transaction_traces_count(&self) -> u32 {
        if self.filtering_applied {
            return self.filtered_transaction_count;
        }
        self.unfiltered_transaction_count
    }

    /// Number of top-level actions that were successfully executed within this block.
    pub fn executed_input_action_count(&self) -> u32 {
        if self.filtering_applied {
            return self.filtered_executed_input_action_count;
        }
        self.unfiltered_executed_input_action_count
    }

    /// Number of actions that were successfully executed within this block.
    pub fn executed_total_action_count(&self) -> u32 {
        if self.filtering_applied {
            return self.filtered_executed_total_action_count;
        }
        self.unfiltered_executed_total_action_count
    }
}
