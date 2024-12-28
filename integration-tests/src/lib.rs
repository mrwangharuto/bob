#![cfg(test)]

mod setup;

use crate::setup::{deploy_bob_canisters, deploy_system_canisters};
use candid::{Nat, Principal};
use ic_ledger_types::{AccountIdentifier, Memo, Tokens, TransferArgs, TransferResult};
use icrc_ledger_types::icrc1::account::Account;
use pocket_ic::{update_candid_as, PocketIcBuilder};
use std::time::SystemTime;

// System canister IDs

pub(crate) const NNS_GOVERNANCE_CANISTER_ID: Principal =
    Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 1, 1, 1]);
pub(crate) const NNS_ICP_LEDGER_CANISTER_ID: Principal =
    Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 2, 1, 1]);
pub(crate) const NNS_ROOT_CANISTER_ID: Principal =
    Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 3, 1, 1]);
pub(crate) const NNS_CYCLES_MINTING_CANISTER_ID: Principal =
    Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 4, 1, 1]);
pub(crate) const NNS_ICP_INDEX_CANISTER_ID: Principal =
    Principal::from_slice(&[0, 0, 0, 0, 0, 0, 0, 0xB, 1, 1]);

// BoB canister IDs

pub(crate) const BOB_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x02, 0x40, 0x00, 0x55, 0x01, 0x01]);
pub(crate) const BOB_LEDGER_CANISTER_ID: Principal =
    Principal::from_slice(&[0x00, 0x00, 0x00, 0x00, 0x02, 0x40, 0x00, 0x59, 0x01, 0x01]);

#[test]
fn spawn_miner() {
    let pic = PocketIcBuilder::new().with_nns_subnet().build();
    pic.set_time(SystemTime::now());
    let user_id = Principal::from_slice(&[0xFF; 29]);
    deploy_system_canisters(&pic, vec![user_id]);
    deploy_bob_canisters(&pic);

    let transfer_args = TransferArgs {
        memo: Memo(1347768404),
        amount: Tokens::from_e8s(100_000_000),
        from_subaccount: None,
        fee: Tokens::from_e8s(10_000),
        to: AccountIdentifier::from_hex(
            "e7b583c3e3e2837c987831a97a6b980cbb0be89819e85915beb3c02006923fce",
        )
        .unwrap(),
        created_at_time: None,
    };
    let block = update_candid_as::<_, (TransferResult,)>(
        &pic,
        NNS_ICP_LEDGER_CANISTER_ID,
        user_id,
        "transfer",
        (transfer_args,),
    )
    .unwrap()
    .0
    .unwrap();

    // wait for the ICP index to sync
    pic.advance_time(std::time::Duration::from_secs(1));
    pic.tick();

    let _miner_id = update_candid_as::<_, (Result<Principal, String>,)>(
        &pic,
        BOB_CANISTER_ID,
        user_id,
        "spawn_miner",
        (block,),
    )
    .unwrap()
    .0
    .unwrap();

    let bob_balance = update_candid_as::<_, (Nat,)>(
        &pic,
        BOB_LEDGER_CANISTER_ID,
        user_id,
        "icrc1_balance_of",
        (Account {
            owner: user_id,
            subaccount: None,
        },),
    )
    .unwrap()
    .0;
    assert_eq!(bob_balance, 0_u64);

    // wait for first BoB block to be mined
    pic.advance_time(std::time::Duration::from_secs(7 * 60));
    for _ in 0..10 {
        pic.tick();
    }

    let bob_balance = update_candid_as::<_, (Nat,)>(
        &pic,
        BOB_LEDGER_CANISTER_ID,
        user_id,
        "icrc1_balance_of",
        (Account {
            owner: user_id,
            subaccount: None,
        },),
    )
    .unwrap()
    .0;
    assert_eq!(bob_balance, 60_000_000_000_u64);

    // wait for second BoB block to be mined
    pic.advance_time(std::time::Duration::from_secs(7 * 60));
    for _ in 0..10 {
        pic.tick();
    }

    let bob_balance = update_candid_as::<_, (Nat,)>(
        &pic,
        BOB_LEDGER_CANISTER_ID,
        user_id,
        "icrc1_balance_of",
        (Account {
            owner: user_id,
            subaccount: None,
        },),
    )
    .unwrap()
    .0;
    assert_eq!(bob_balance, 120_000_000_000_u64);
}
