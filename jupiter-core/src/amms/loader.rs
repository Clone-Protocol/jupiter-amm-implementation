use std::{collections::HashSet, str::FromStr};

use anyhow::{anyhow, Result};
use jupiter_amm_interface::{Amm, KeyedAccount};
use solana_sdk::pubkey::Pubkey;
use jupiter_interface::CloneInterface;

use super::spl_token_swap_amm::{SplTokenSwapAmm, SPL_TOKEN_SWAP_PROGRAMS};

pub fn amm_factory(
    keyed_account: &KeyedAccount,
    _saber_wrapper_mints: &mut HashSet<Pubkey>,
) -> Result<Box<dyn Amm + Send + Sync>> {
    let owner = keyed_account.account.owner;

    let clone_owner = Pubkey::from_str("C1onEW2kPetmHmwe74YC1ESx3LnFEpVau6g2pg4fHycr").unwrap();
    // Add your AMM here
    if owner.eq(&clone_owner) {
        Ok(Box::new(
            CloneInterface::from_keyed_account(keyed_account)?
        ))
    } else if SPL_TOKEN_SWAP_PROGRAMS.contains_key(&owner) {
        Ok(Box::new(SplTokenSwapAmm::from_keyed_account(
            keyed_account,
        )?))
    } else {
        Err(anyhow!(
            "Unsupported pool {}, from owner {}",
            keyed_account.key,
            keyed_account.account.owner
        ))
    }
}
