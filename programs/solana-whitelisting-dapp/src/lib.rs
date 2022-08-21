use anchor_lang::prelude::*;
use anchor_lang::solana_program::account_info::AccountInfo;

declare_id!("DoXHuZ7cuGeDiLV6AwnoEsMLo1UGZ3AX5Mk7KgEy4UwV");

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBKEY_LENGTH: usize = 32;

// business logics
#[program]
pub mod whitelisting {
    use super::*;

    // create a whitelist config account for a use case specified by the input seed
    pub fn create_whitelist_config(ctx: Context<CreateWhitelistConfig>, _seed: String) -> Result<()> {
        let whitelist_config = &mut ctx.accounts.whitelist_config;
        whitelist_config.authority = ctx.accounts.authority.key();
        Ok(())
    }

    // add a wallet address to the whitelist
    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>) -> Result<()> {
        let whitelist_data = &mut ctx.accounts.whitelist_data;
        whitelist_data.whitelisted_data = ctx.accounts.wallet.key();
        Ok(())
    }

    pub fn remove_from_whitelist(_ctx: Context<RemoveFromWhitelist>, _seed: String) -> Result<()> {
        Ok(())
    }
}

// data validators
#[derive(Accounts)]
#[instruction(seed: String)]
pub struct CreateWhitelistConfig<'info> {
    // initialize an account on solana chain
    #[account(
        init, 
        payer = authority, 
        space = WhitelistConfig::LEN, 
        seeds = [
            // pass in whitelist use case string
            &seed.as_bytes(), 
            authority.key().as_ref(), 
        ],
        bump
    )]
    pub whitelist_config: Account<'info, WhitelistConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    // security check to see if the caller has the authority
    #[account(mut, has_one=authority)]
    pub whitelist_config: Account<'info, WhitelistConfig>,
    /// CHECK: the wallet is passed in as a pubkey
    pub wallet: UncheckedAccount<'info>,
    #[account(mut)] 
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = WhitelistData::LEN,
        seeds = [
            whitelist_config.key().as_ref(),
            wallet.key().as_ref(),
        ],
        bump
    )]
    pub whitelist_data: Account<'info, WhitelistData>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(seed: String)]
pub struct RemoveFromWhitelist<'info> {
    #[account(
        mut, 
        seeds = [
            authority.key().as_ref(), 
            seed.as_bytes()
            ], 
        bump,
        has_one = authority
    )]
    pub whitelist_config: Account<'info, WhitelistConfig>,
    #[account(
        mut, 
        seeds = [
            authority.key().as_ref(), 
            whitelist_data.key().as_ref(), 
            ], 
        bump, 
        close=authority
    )]
    pub whitelist_data: Account<'info, WhitelistData>,
    /// CHECK: wallet pubkey
    pub whitelisted_data: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

// data structures
#[account]
pub struct WhitelistConfig {
    pub authority: Pubkey
}


impl WhitelistConfig {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBKEY_LENGTH;
}

#[account]
pub struct WhitelistData {
    pub whitelisted_data: Pubkey
}

impl WhitelistData {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBKEY_LENGTH;

}
