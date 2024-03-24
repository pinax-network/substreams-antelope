use crate::block::pb::TransactionStatus::TransactionstatusExecuted;
use crate::pb;

impl pb::Block {
    /// returns all executed transaction traces from the block without consuming it.
    /// ```ignore
    /// let transactions = block.transaction_traces()
    ///     .map(|trx| {
    ///         // your transaction logic
    ///     })
    ///     .collect();
    /// ```
    pub fn transaction_traces(&self) -> impl Iterator<Item = &pb::TransactionTrace> {
        let traces = if self.filtering_applied {
            self.filtered_transaction_traces.iter()
        } else {
            self.unfiltered_transaction_traces.iter()
        };

        traces.filter(|trx| trx.receipt.is_some() && trx.receipt.as_ref().unwrap().status == TransactionstatusExecuted as i32)
    }

    /// returns all executed transaction traces from the block and consumes it.
    /// ```ignore
    /// let transactions = block.into_transaction_traces()
    ///     .map(|trx| {
    ///         // your transaction logic
    ///     })
    ///     .collect();
    /// ```
    pub fn into_transaction_traces(self) -> impl Iterator<Item = pb::TransactionTrace> {
        let traces = if self.filtering_applied {
            self.filtered_transaction_traces.into_iter()
        } else {
            self.unfiltered_transaction_traces.into_iter()
        };

        traces.filter(|trx| trx.receipt.is_some() && trx.receipt.as_ref().unwrap().status == TransactionstatusExecuted as i32)
    }

    /// returns all executed action traces along with related transaction traces from the block without consuming it
    /// ```ignore
    /// let actions = block.action_traces()
    ///     .map(|(action, trx)| {
    ///         // your action logic
    ///     })
    ///     .collect();
    /// ```
    pub fn action_traces(&self) -> impl Iterator<Item = (&pb::ActionTrace, &pb::TransactionTrace)> {
        self.transaction_traces().flat_map(|trx| trx.action_traces.iter().map(move |trace| (trace, trx)))
    }

    /// returns all executed action traces from the block and consumes it. Transaction traces are not returned.
    /// ```ignore
    /// let actions = block.into_action_traces()
    ///     .map(|action| {
    ///         // your action logic
    ///     })
    ///     .collect();
    /// ```
    pub fn into_action_traces(self) -> impl Iterator<Item = pb::ActionTrace> {
        self.into_transaction_traces().flat_map(|trx| trx.action_traces)
    }

    /// returns all executed actions and notifications of a specified type from the block which match the given contract accounts, or all if account is empty
    /// ```ignore
    /// let actions = block.all_actions::<abi::contract::actions::Statelog>(&["mycontract"])
    ///     .map(|(action, trace, trx)| StateChange {
    ///         // set action fields
    ///     })
    ///     .collect();
    /// ```
    pub fn all_actions<'a, A: crate::action::Action>(&'a self, accounts: &'a [&str]) -> impl Iterator<Item = (A, &pb::ActionTrace, &pb::TransactionTrace)> + 'a {
        self.action_traces().filter_map(|(trace, trx)| {
            let contract = trace.action.as_ref().unwrap().account.as_str();
            if !accounts.is_empty() && !accounts.contains(&contract) {
                return None;
            }

            A::match_and_decode(trace).map(|action| (action, trace, trx))
        })
    }

    /// returns all actions of a specified type from the block which match the given contract accounts
    /// NOT including action notifications
    /// ```ignore
    /// let actions = block.actions::<abi::contract::actions::Statelog>(&["mycontract"])
    ///     .map(|(action, trx)| StateChange {
    ///         // set action fields
    ///     })
    ///     .collect();
    /// ```
    pub fn actions<'a, A: crate::action::Action>(&'a self, accounts: &'a [&str]) -> impl Iterator<Item = (A, &pb::ActionTrace, &pb::TransactionTrace)> + 'a {
        self.all_actions(accounts)
            .filter(|(_, trace, _)| trace.receiver.as_str() == trace.action.as_ref().unwrap().account.as_str())
    }

    /// returns all action notifications of a specified type from the block which match the given contract accounts
    /// ONLY including action notifications
    /// ```ignore
    /// let notifications = block.notifications::<abi::contract::actions::Statelog>(&["mycontract"])
    ///     .map(|(action, trx)| StateChange {
    ///         // set action fields
    ///     })
    ///     .collect();
    /// ```
    pub fn notifications<'a, A: crate::action::Action>(&'a self, accounts: &'a [&str]) -> impl Iterator<Item = (A, &pb::ActionTrace, &pb::TransactionTrace)> + 'a {
        self.all_actions(accounts)
            .filter(|(_, trace, _)| trace.receiver.as_str() != trace.action.as_ref().unwrap().account.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::pb::TransactionStatus::{TransactionstatusExecuted, TransactionstatusSoftfail};

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
        let block = create_test_block(false, unfiltered_traces.clone(), vec![], 2, 0, 5, 0, 7, 0);

        let all_traces: Vec<_> = block.into_transaction_traces().collect();
        assert_eq!(all_traces, unfiltered_traces);
    }

    #[test]
    fn test_all_action_traces() {
        let unfiltered_traces = vec![
            pb::TransactionTrace {
                id: String::from("trx1"),
                receipt: Some(pb::TransactionReceiptHeader {
                    status: TransactionstatusExecuted as i32,
                    ..Default::default()
                }),
                action_traces: vec![pb::ActionTrace { ..Default::default() }],
                ..Default::default()
            },
            pb::TransactionTrace {
                id: String::from("trx2"),
                receipt: Some(pb::TransactionReceiptHeader {
                    status: TransactionstatusExecuted as i32,
                    ..Default::default()
                }),
                action_traces: vec![pb::ActionTrace { ..Default::default() }, pb::ActionTrace { ..Default::default() }],
                ..Default::default()
            },
        ];
        let block = create_test_block(false, unfiltered_traces.clone(), vec![], 2, 0, 5, 0, 7, 0);

        let all_traces: Vec<_> = block.action_traces().collect();
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

        let produced_traces: Vec<_> = block.transaction_traces().cloned().collect();
        assert_eq!(executed_traces, produced_traces);
    }

    #[test]
    fn test_executed_transaction_traces_no_receipt() {
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
                    status: TransactionstatusSoftfail as i32,
                    ..Default::default()
                }),
                ..Default::default()
            },
            pb::TransactionTrace {
                id: String::from("trx3"),
                receipt: None,
                ..Default::default()
            },
        ];
        let block = create_test_block(false, executed_traces.clone(), vec![], 2, 0, 5, 0, 7, 0);

        let produced_traces: Vec<_> = block.transaction_traces().cloned().collect();
        assert_eq!(executed_traces[0], produced_traces[0]);
        assert_eq!(produced_traces.len(), 1);
    }
}
