use crate::candid::InitArg;
use crate::state::State;

mod manage_canister {
    use crate::scheduler::test_fixtures::{usdc, usdc_metadata, usdt, usdt_metadata};
    use crate::state::tests::{expect_panic_with_message, init_state};
    use crate::state::{
        Canisters, Index, Ledger, ManageSingleCanister, ManagedCanisterStatus, WasmHash,
    };
    use candid::Principal;
    use std::fmt::Debug;

    #[test]
    fn should_record_created_canister_in_any_order() {
        let mut state = init_state();
        state.record_new_erc20_token(usdc(), usdc_metadata());
        let usdc_index_canister_id = Principal::from_slice(&[1_u8; 29]);
        state.record_created_canister::<Index>(&usdc(), usdc_index_canister_id);
        assert_eq!(
            state.managed_status::<Index>(&usdc()),
            Some(&ManagedCanisterStatus::Created {
                canister_id: usdc_index_canister_id
            })
        );
        let usdc_ledger_canister_id = Principal::from_slice(&[2_u8; 29]);
        assert_ne!(usdc_index_canister_id, usdc_ledger_canister_id);
        state.record_created_canister::<Ledger>(&usdc(), usdc_ledger_canister_id);
        assert_eq!(
            state.managed_status::<Ledger>(&usdc()),
            Some(&ManagedCanisterStatus::Created {
                canister_id: usdc_ledger_canister_id
            })
        );

        state.record_new_erc20_token(usdt(), usdt_metadata());
        let usdt_ledger_canister_id = Principal::from_slice(&[3_u8; 29]);
        state.record_created_canister::<Ledger>(&usdt(), usdt_ledger_canister_id);
        assert_eq!(
            state.managed_status::<Ledger>(&usdt()),
            Some(&ManagedCanisterStatus::Created {
                canister_id: usdt_ledger_canister_id
            })
        );
        let usdt_index_canister_id = Principal::from_slice(&[4_u8; 29]);
        state.record_created_canister::<Index>(&usdt(), usdt_index_canister_id);
        assert_eq!(
            state.managed_status::<Index>(&usdt()),
            Some(&ManagedCanisterStatus::Created {
                canister_id: usdt_index_canister_id
            })
        );
    }

    #[test]
    fn should_record_installed_canister_and_keep_correct_status() {
        fn test<C: Debug>()
        where
            Canisters: ManageSingleCanister<C>,
        {
            let mut state = init_state();
            let canister_id = Principal::from_slice(&[1_u8; 29]);
            let contract = usdc();

            assert_eq!(state.managed_status::<C>(&contract), None);

            state.record_new_erc20_token(contract.clone(), usdc_metadata());
            state.record_created_canister::<C>(&contract, canister_id);
            assert_eq!(
                state.managed_status::<C>(&contract),
                Some(&ManagedCanisterStatus::Created { canister_id })
            );

            let wasm_hash = WasmHash::from([1_u8; 32]);
            state.record_installed_canister::<C>(&contract, wasm_hash.clone());
            assert_eq!(
                state.managed_status::<C>(&contract),
                Some(&ManagedCanisterStatus::Installed {
                    canister_id,
                    installed_wasm_hash: wasm_hash
                })
            );
        }

        test::<Index>();
        test::<Ledger>();
    }

    #[test]
    fn should_panic_when_recording_created_canister_for_not_managed_erc20_token() {
        fn test<C: Debug>()
        where
            Canisters: ManageSingleCanister<C>,
        {
            let mut state = init_state();

            expect_panic_with_message(
                || state.record_created_canister::<C>(&usdc(), Principal::from_slice(&[1_u8; 29])),
                "not managed",
            );
        }

        test::<Index>();
        test::<Ledger>();
    }

    #[test]
    fn should_panic_when_recording_twice_same_new_erc20_token() {
        let mut state = init_state();
        let erc20 = usdc();
        state.record_new_erc20_token(erc20.clone(), usdc_metadata());

        expect_panic_with_message(
            || state.record_new_erc20_token(erc20, usdc_metadata()),
            "already managed",
        );
    }

    #[test]
    fn should_panic_when_recording_twice_canister_created() {
        fn test<C: Debug>()
        where
            Canisters: ManageSingleCanister<C>,
        {
            let mut state = init_state();
            let erc20 = usdc();
            state.record_new_erc20_token(erc20.clone(), usdc_metadata());
            let canister_id = Principal::from_slice(&[1_u8; 29]);
            state.record_created_canister::<C>(&erc20, canister_id);

            expect_panic_with_message(
                || state.record_created_canister::<C>(&erc20, canister_id),
                "already created",
            );
        }

        test::<Index>();
        test::<Ledger>();
    }

    #[test]
    fn should_panic_when_recording_installed_canister_but_canister_was_not_created() {
        fn test<C: Debug>()
        where
            Canisters: ManageSingleCanister<C>,
        {
            let mut state = init_state();

            expect_panic_with_message(
                || state.record_installed_canister::<C>(&usdc(), WasmHash::from([1_u8; 32])),
                "no managed canisters",
            );
        }

        test::<Index>();
        test::<Ledger>();
    }
}

mod wasm_hash {
    use crate::state::WasmHash;
    use assert_matches::assert_matches;
    use proptest::arbitrary::any;
    use proptest::array::uniform32;
    use proptest::{prop_assert_eq, proptest};
    use std::str::FromStr;

    proptest! {
        #[test]
        fn should_decode_display_string(hash in uniform32(any::<u8>())) {
            let parsed_hash = WasmHash::from_str(&WasmHash::from(hash).to_string()).unwrap();
            prop_assert_eq!(parsed_hash.as_ref(), &hash);
        }

        #[test]
        fn should_error_on_invalid_hash(invalid_hash in "[0-9a-fA-F]{0,63}|[0-9a-fA-F]{65,}") {
           assert_matches!(WasmHash::from_str(&invalid_hash), Err(_));
        }

         #[test]
        fn should_accept_valid_hash(valid_hash in "[0-9a-fA-F]{64}") {
            let result = WasmHash::from_str(&valid_hash).unwrap();
            prop_assert_eq!(result.as_ref(), &hex::decode(valid_hash).unwrap()[..]);
        }
    }
}

fn expect_panic_with_message<F: FnOnce() -> R, R: std::fmt::Debug>(f: F, expected_message: &str) {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(f));
    let error = result.unwrap_err();
    let panic_message = {
        if let Some(s) = error.downcast_ref::<String>() {
            s.to_string()
        } else if let Some(s) = error.downcast_ref::<&str>() {
            s.to_string()
        } else {
            format!("{:?}", error)
        }
    };
    assert!(
        panic_message.contains(expected_message),
        "Expected panic message to contain: {}, but got: {}",
        expected_message,
        panic_message
    );
}

fn init_state() -> State {
    State::from(InitArg {})
}
