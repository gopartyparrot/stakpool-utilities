use rust_decimal::{prelude::FromPrimitive, Decimal};
use solana_client::rpc_client::RpcClient;
use spl_stake_pool::{
    solana_program::{borsh::try_from_slice_unchecked, pubkey::Pubkey},
    state::StakePool,
};
use std::{ops::Div, str::FromStr};

//Note: if you need js sdk, see: @solana/spl-stake-pool
fn main() {
    let url = "https://api.mainnet-beta.solana.com".to_string();
    let client = RpcClient::new(url);

    let stake_pool_pubkey =
        Pubkey::from_str("AMjGNE12gNoZnrU68AGxUibYEjrGPgpPk3EYG5MZCiZQ").unwrap(); //parrot stake pool address
    let account = client.get_account(&stake_pool_pubkey).unwrap();
    // println!("stake pool owner: {}", account.owner); //should be parrot stake pool program: 3puRp4bBPqDyBJuumc4Nwrv5W699kCZpmoTaQQKaobJh

    let pool = try_from_slice_unchecked::<StakePool>(&account.data[..]).unwrap();
    let staked_lamports = Decimal::from_u64(pool.total_stake_lamports).unwrap();
    let supply = Decimal::from_u64(pool.pool_token_supply).unwrap();

    println!("price:");
    println!("    1 prtSOL = {} SOL", staked_lamports.div(supply))
}
