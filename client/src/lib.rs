// This is necessary because clippy throws 'unneeded unit expression' error
// on the wasm_bindgen expressions
#![allow(clippy::unused_unit)]

mod get_auction;
mod get_current_cycle;
mod get_top_bidder;
mod try_find_master;

use agsol_gold_contract::instruction::factory::*;
use agsol_gold_contract::pda::{
    auction_pool_seeds, auction_root_state_seeds, secondary_pool_seeds,
};
use agsol_gold_contract::solana_program;
use agsol_gold_contract::solana_program::pubkey::Pubkey;
use agsol_gold_contract::ID as GOLD_ID;
use agsol_wasm_client::rpc_config::{CommitmentLevel, Encoding, RpcConfig};
use agsol_wasm_client::{wasm_instruction, Net};
use borsh::BorshSerialize;
use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[cfg(feature = "Devnet")]
const NET: Net = Net::Devnet;
#[cfg(not(feature = "Devnet"))]
const NET: Net = Net::Mainnet;

const RPC_CONFIG: RpcConfig = RpcConfig {
    encoding: Some(Encoding::JsonParsed),
    commitment: Some(CommitmentLevel::Processed),
};

wasm_instruction!(initialize_auction);
wasm_instruction!(delete_auction);
wasm_instruction!(place_bid);
wasm_instruction!(claim_funds);

#[wasm_bindgen(js_name = "getAuctionWasm")]
pub async fn get_auction_wasm(auction_id: String) -> Result<Uint8Array, JsValue> {
    let auction = get_auction::get_auction(auction_id)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    Ok(Uint8Array::from(auction.try_to_vec().unwrap().as_slice()))
}

#[wasm_bindgen(js_name = "getAuctionCycleStateWasm")]
pub async fn get_auction_cycle_state_wasm(
    root_state_pubkey: Pubkey,
    cycle_num: u64,
) -> Result<Uint8Array, JsValue> {
    let auction_cycle_state = get_auction::get_auction_cycle_state(&root_state_pubkey, cycle_num)
        .await
        .map_err(|e| JsValue::from(e.to_string()))?;

    Ok(Uint8Array::from(
        auction_cycle_state.try_to_vec().unwrap().as_slice(),
    ))
}

#[wasm_bindgen(js_name = "getTopBidderWasm")]
pub async fn get_top_bidder_wasm(auction_id: String) -> Result<Pubkey, JsValue> {
    get_top_bidder::get_top_bidder(auction_id)
        .await
        .map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen(js_name = "getCurrentCycleWasm")]
pub async fn get_current_cycle_wasm(auction_id: String) -> Result<u64, JsValue> {
    get_current_cycle::get_current_cycle(auction_id)
        .await
        .map_err(|e| JsValue::from(e.to_string()))
}

#[wasm_bindgen(js_name = "getAuctionPoolPubkeyWasm")]
pub fn wasm_auction_pool_pubkey(secondary: bool) -> Pubkey {
    let seeds = if secondary {
        secondary_pool_seeds()
    } else {
        auction_pool_seeds()
    };
    let (auction_pool_pubkey, _) = Pubkey::find_program_address(&seeds, &GOLD_ID);
    auction_pool_pubkey
}

#[wasm_bindgen(js_name = "getAuctionRootStatePubkeyWasm")]
pub fn wasm_auction_root_state_pubkey(auction_id: &[u8]) -> Pubkey {
    let (auction_root_state_pubkey, _) =
        Pubkey::find_program_address(&auction_root_state_seeds(auction_id), &GOLD_ID);
    auction_root_state_pubkey
}

#[wasm_bindgen(js_name = "isIdUniqueWasm")]
pub async fn wasm_is_id_unique(auction_id: String) -> bool {
    try_find_master::try_find_master(auction_id).await.is_err()
}

#[wasm_bindgen(js_name = "getNetWasm")]
pub fn wasm_get_net() -> String {
    NET.to_url().to_owned()
}
