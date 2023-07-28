use crate::block::pb::TransactionStatus::TransactionstatusExecuted;
use crate::{pb, ActionTrace};

impl pb::Block {
    /// returns all transaction traces from the block without consuming it.
    pub fn all_transaction_traces(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        if self.filtering_applied {
            self.filtered_transaction_traces.iter()
        } else {
            self.unfiltered_transaction_traces.iter()
        }
    }

    /// returns all transaction traces from the block and consumes it.
    pub fn into_all_transaction_traces(self) -> impl Iterator<Item = pb::TransactionTrace> {
        if self.filtering_applied {
            self.filtered_transaction_traces.into_iter()
        } else {
            self.unfiltered_transaction_traces.into_iter()
        }
    }

    /// returns all action traces from the block without consuming it
    pub fn all_action_traces(&self) -> impl Iterator<Item = &pb::ActionTrace> {
        self.all_transaction_traces().flat_map(|trx| &trx.action_traces)
    }

    /// returns all action traces from the block and consumes it
    pub fn into_all_action_traces(self) -> impl Iterator<Item = pb::ActionTrace> {
        self.into_all_transaction_traces().flat_map(|trx| trx.action_traces)
    }

    /// returns all transaction traces which have the status `executed`
    pub fn executed_transaction_traces(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        self.all_transaction_traces()
            .filter(|trx| trx.receipt.as_ref().unwrap().status == TransactionstatusExecuted as i32)
    }

    /// returns the number of transaction traces included in this block
    pub fn transaction_traces_count(&self) -> u32 {
        if self.filtering_applied {
            self.filtered_transaction_count
        } else {
            self.unfiltered_transaction_count
        }
    }

    /// Number of top-level actions that were successfully executed within this block.
    pub fn executed_input_action_count(&self) -> u32 {
        if self.filtering_applied {
            self.filtered_executed_input_action_count
        } else {
            self.unfiltered_executed_input_action_count
        }
    }

    /// Number of actions that were successfully executed within this block.
    pub fn executed_total_action_count(&self) -> u32 {
        if self.filtering_applied {
            self.filtered_executed_total_action_count
        } else {
            self.unfiltered_executed_total_action_count
        }
    }

    /// returns all actions of a type from the block which match the given accounts
    /// Example:
    /// ```no_run
    /// let state_changes = block.actions::<abi::contract::actions::Statelog>(&["mycontract"])
    ///     .map(|(action, trx)| StateChange {
    ///         trx_id: trx.transaction_id.clone(),
    ///         trx_index: trx.action_ordinal,
    ///         user: action.user,
    ///         state: action.status,
    ///     })
    ///     .collect()
    /// ```
    pub fn actions<'a, A: crate::action::Action> (
        &'a self,
        accounts: &'a [&str],
    ) -> impl Iterator<Item = (A, &ActionTrace)> + 'a
    {
        self.all_action_traces().filter_map(|trace| {
            if !accounts.contains(&trace.action.as_ref().unwrap().account.as_str()) {
                return None;
            }

            A::match_and_decode(trace).map(|action| (action, trace))
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::pb::TransactionStatus::TransactionstatusExecuted;

    // Helper function to create a test pb::Block instance
    fn create_test_block(
        filtering_applied: bool,
        unfiltered_transaction_traces: Vec<pb::TransactionTrace>,
        filtered_transaction_traces: Vec<pb::TransactionTrace>,
        unfiltered_transaction_count: u32,
        filtered_transaction_count: u32,
        unfiltered_executed_input_action_count: u32,
        filtered_executed_input_action_count: u32,
        unfiltered_executed_total_action_count: u32,
        filtered_executed_total_action_count: u32,
    ) -> pb::Block {
        pb::Block {
            filtering_applied,
            unfiltered_transaction_traces,
            filtered_transaction_traces,
            unfiltered_transaction_count,
            filtered_transaction_count,
            unfiltered_executed_input_action_count,
            filtered_executed_input_action_count,
            unfiltered_executed_total_action_count,
            filtered_executed_total_action_count,
            ..Default::default()
        }
    }

    #[test]
    fn test_all_transaction_traces() {
        let unfiltered_traces = vec![
            pb::TransactionTrace {
                id: String::from("trx1"),
                ..Default::default()
            },
            pb::TransactionTrace {
                id: String::from("trx2"),
                ..Default::default()
            },
        ];
        let block = create_test_block(false, unfiltered_traces.clone(), vec![], 2, 0, 5, 0, 7, 0);

        let all_traces: Vec<_> = block.into_all_transaction_traces().collect();
        assert_eq!(all_traces, unfiltered_traces);
    }

    #[test]
    fn test_all_action_traces() {
        let unfiltered_traces = vec![
            pb::TransactionTrace {
                id: String::from("trx1"),
                action_traces: vec![
                    pb::ActionTrace {
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            pb::TransactionTrace {
                id: String::from("trx2"),
                action_traces: vec![
                    pb::ActionTrace {
                        ..Default::default()
                    },
                    pb::ActionTrace {
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ];
        let block = create_test_block(false, unfiltered_traces.clone(), vec![], 2, 0, 5, 0, 7, 0);

        let all_traces: Vec<_> = block.all_action_traces().collect();
        assert_eq!(all_traces.len(), 3);
    }

    #[test]
    fn test_executed_transaction_traces() {
        let executed_traces = vec![
            pb::TransactionTrace {
                id: String::from("trx1"),
                receipt: Some(pb::TransactionReceiptHeader {
                    status: TransactionstatusExecuted as i32,
                    ..Default::default()
                }),
                ..Default::default()
            },
            pb::TransactionTrace {
                id: String::from("trx2"),
                receipt: Some(pb::TransactionReceiptHeader {
                    status: TransactionstatusExecuted as i32,
                    ..Default::default()
                }),
                ..Default::default()
            },
        ];
        let block = create_test_block(false, executed_traces.clone(), vec![], 2, 0, 5, 0, 7, 0);

        let executed_traces: Vec<_> = block.executed_transaction_traces().collect();
        assert_eq!(executed_traces, executed_traces);
    }

    #[test]
    fn test_transaction_traces_count() {
        let block = create_test_block(false, vec![], vec![], 8, 0, 5, 0, 7, 0);

        assert_eq!(block.transaction_traces_count(), 8);
    }

    #[test]
    fn test_executed_input_action_count() {
        let block = create_test_block(false, vec![], vec![], 0, 0, 9, 0, 7, 0);

        assert_eq!(block.executed_input_action_count(), 9);
    }

    #[test]
    fn test_executed_total_action_count() {
        let block = create_test_block(false, vec![], vec![], 0, 0, 5, 0, 11, 0);

        assert_eq!(block.executed_total_action_count(), 11);
    }
}
