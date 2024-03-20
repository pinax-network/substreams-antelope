#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    use substreams_antelope::pb;
    use substreams_antelope_abigen::Abigen;
    use pb::TransactionStatus::TransactionstatusExecuted;

    mod actions {
        use substreams_antelope_abigen::decoder::decode;
        use substreams_antelope_abigen::types::*;
        #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct Transfer {
            pub from: Name,
            pub to: Name,
            pub quantity: Asset,
            pub memo: String,
        }
        impl substreams_antelope::Action for Transfer {
            const NAME: &'static str = "transfer";
            fn decode(trace: &substreams_antelope::pb::ActionTrace) -> Result<Self, substreams_antelope::Error> {
                Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
            }
        }

        #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct Transfer2 {
            pub from: Name,
            pub to: Name,
            pub quantity: Asset,
        }
        impl substreams_antelope::Action for Transfer2 {
            const NAME: &'static str = "transfer2";
            fn decode(trace: &substreams_antelope::pb::ActionTrace) -> Result<Self, substreams_antelope::Error> {
                Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
            }
        }
    }

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

    fn create_transfer_trace() -> pb::TransactionTrace {
        pb::TransactionTrace {
            id: String::from("trx1"),
            receipt: Some(pb::TransactionReceiptHeader {
                status: TransactionstatusExecuted as i32,
                ..Default::default()
            }),
            action_traces: vec![
                // transfer action
                pb::ActionTrace {
                    action: Some(pb::Action {
                        account: "tokencontract".into(),
                        name: "transfer".into(),
                        json_data: r#"{"from": "acc1", "to": "acc2", "quantity": "1.0000 EOS", "memo": "hello"}"#.into(),
                        ..Default::default()
                    }),
                    receiver: "tokencontract".into(),
                    ..Default::default()
                },
                // notification #1
                pb::ActionTrace {
                    action: Some(pb::Action {
                        account: "tokencontract".into(),
                        name: "transfer".into(),
                        json_data: r#"{"from": "acc1", "to": "acc2", "quantity": "1.0000 EOS", "memo": "hello"}"#.into(),
                        ..Default::default()
                    }),
                    receiver: "acc1".into(),
                    ..Default::default()
                },
                // notification #2
                pb::ActionTrace {
                    action: Some(pb::Action {
                        account: "tokencontract".into(),
                        name: "transfer".into(),
                        json_data: r#"{"from": "acc1", "to": "acc2", "quantity": "1.0000 EOS", "memo": "hello"}"#.into(),
                        ..Default::default()
                    }),
                    receiver: "acc2".into(),
                    ..Default::default()
                },
                // action from another token contract
                pb::ActionTrace {
                    action: Some(pb::Action {
                        account: "othercontract".into(),
                        name: "transfer".into(),
                        json_data: r#"{"from": "acc1", "to": "acc2", "quantity": "1.0000 EOS", "memo": "hello"}"#.into(),
                        ..Default::default()
                    }),
                    receiver: "othercontract".into(),
                    ..Default::default()
                },
                // transfer2 action without memo
                pb::ActionTrace {
                    action: Some(pb::Action {
                        account: "tokencontract".into(),
                        name: "transfer2".into(),
                        json_data: r#"{"from": "acc1", "to": "acc2", "quantity": "1.0000 EOS"}"#.into(),
                        ..Default::default()
                    }),
                    receiver: "tokencontract".into(),
                    ..Default::default()
                },
                // transfer2 notification without memo
                pb::ActionTrace {
                    action: Some(pb::Action {
                        account: "tokencontract".into(),
                        name: "transfer2".into(),
                        json_data: r#"{"from": "acc1", "to": "acc2", "quantity": "1.0000 EOS"}"#.into(),
                        ..Default::default()
                    }),
                    receiver: "acc2".into(),
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    }

    #[test]
    fn test_contract_generation() {
        let abi_jsons_dir = "abi/";
        let mut files_tested = 0;
        for entry in fs::read_dir(abi_jsons_dir).expect("failed to read ABI JSON directory") {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "json" {
                        let path = path.to_string_lossy();
                        let generated = Abigen::new("Contract", &path)
                            .expect(&format!("failed to read ABI JSON for {}", path))
                            .generate()
                            .expect(&format!("failed to generate contract for {}", path))
                            .get_code()
                            .to_string();

                        let golden_file_path =
                            format!("{}{}.rs.golden", abi_jsons_dir, entry.path().file_stem().unwrap().to_string_lossy());
                        let mut golden_contents = String::new();
                        fs::File::open(&golden_file_path)
                            .and_then(|mut file| file.read_to_string(&mut golden_contents))
                            .expect(&format!("failed to read golden file {golden_file_path}"));

                        pretty_assertions::assert_eq!(generated, golden_contents, "\n\n❌ Testing {path} against {golden_file_path}");
                        files_tested += 1;
                    }
                }
            }
        }
        assert!(files_tested > 0, "No files tested");
    }

    #[test]
    fn test_actions() {
        let transfer_trace = create_transfer_trace();
        let block = create_test_block(false, vec![transfer_trace.clone()], vec![], 2, 0, 5, 0, 7, 0);
        let actions: Vec<_> = block.actions::<actions::Transfer>(&["tokencontract"]).collect();
        pretty_assertions::assert_eq!(
            actions,
            vec![(
                actions::Transfer {
                    from: "acc1".into(),
                    to: "acc2".into(),
                    quantity: "1.0000 EOS".into(),
                    memo: "hello".into(),
                },
                &transfer_trace.action_traces[0]
            ),]
        );
        pretty_assertions::assert_eq!(
            actions.len(),
            1
        );
    }

    #[test]
    fn test_actions_2() {
        let transfer_trace = create_transfer_trace();
        let block = create_test_block(false, vec![transfer_trace.clone()], vec![], 2, 0, 5, 0, 7, 0);
        let actions: Vec<_> = block.actions::<actions::Transfer2>(&["tokencontract"]).collect();
        pretty_assertions::assert_eq!(
            actions,
            vec![(
                actions::Transfer2 {
                    from: "acc1".into(),
                    to: "acc2".into(),
                    quantity: "1.0000 EOS".into(),
                },
                &transfer_trace.action_traces[4]
            ),]
        );
        pretty_assertions::assert_eq!(
            actions.len(),
            1
        );
    }

    #[test]
    fn test_notifications() {
        let transfer_trace = create_transfer_trace();
        let block = create_test_block(false, vec![transfer_trace.clone()], vec![], 2, 0, 5, 0, 7, 0);
        let actions: Vec<_> = block.notifications::<actions::Transfer>(&["tokencontract"]).collect();
        pretty_assertions::assert_eq!(
            actions,
            vec![
                (
                    actions::Transfer {
                        from: "acc1".into(),
                        to: "acc2".into(),
                        quantity: "1.0000 EOS".into(),
                        memo: "hello".into(),
                    },
                    &transfer_trace.action_traces[1]
                ),
                (
                    actions::Transfer {
                        from: "acc1".into(),
                        to: "acc2".into(),
                        quantity: "1.0000 EOS".into(),
                        memo: "hello".into(),
                    },
                    &transfer_trace.action_traces[2]
                ),
            ]
        );
    }

    #[test]
    fn test_notifications_2() {
        let transfer_trace = create_transfer_trace();
        let block = create_test_block(false, vec![transfer_trace.clone()], vec![], 2, 0, 5, 0, 7, 0);
        let actions: Vec<_> = block.notifications::<actions::Transfer2>(&["tokencontract"]).collect();
        pretty_assertions::assert_eq!(
            actions,
            vec![
                (
                    actions::Transfer2 {
                        from: "acc1".into(),
                        to: "acc2".into(),
                        quantity: "1.0000 EOS".into(),
                    },
                    &transfer_trace.action_traces[5]
                ),
            ]
        );
    }
}
