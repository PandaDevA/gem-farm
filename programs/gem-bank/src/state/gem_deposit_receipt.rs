use anchor_lang::prelude::*;
use jet_proc_macros::assert_size;

/// GDR is necessary to locate all gem boxes for a given bank/vault
#[assert_size(104)]
#[account]
pub struct GemDepositReceipt {
    // each gem gox sits inside a single vault
    pub vault: Pubkey,

    // the token account that actually holds the deposited gem(s)
    pub gem_box_address: Pubkey,

    // the following is really stored for convenience, so we don't have to fetch gem account separately
    pub gem_mint: Pubkey,

    pub gem_amount: u64,
}
