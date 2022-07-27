use anchor_lang::prelude::*;
use anchor_lang::solana_program::account_info::AccountInfo;

declare_id!("GgcfkCKzrDHHnuhfAEvC7ohNjZk6hY9KZW2t9Sk7neRX");

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBKEY_LENGTH: usize = 32;

// business logics
#[program]
pub mod whitelisting {
    use super::*;

    // create a base whitelist for a use case specified by the user
    pub fn create_whitelist(ctx: Context<CreateWhitelist>, _key: String) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelist;
        whitelist.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>) -> Result<()> {
        let whitelist_data = &mut ctx.accounts.whitelist_data;
        whitelist_data.whitelisted_data = ctx.accounts.wallet.key();
        Ok(())
    }
}

// data validators
#[derive(Accounts)]
#[instruction(key: String)]
pub struct CreateWhitelist<'info> {
    #[account(
        init, 
        payer = authority, 
        space = Whitelist::LEN, 
        seeds = [
            // pass in whitelist use case string
            &key.as_bytes(), 
            authority.key().as_ref(), 
        ],
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    #[account(mut, has_one=authority)]
    pub whitelist: Account<'info, Whitelist>,
    /// CHECK: TODO
    pub wallet: UncheckedAccount<'info>, 
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = WhitelistData::LEN,
        seeds = [
            whitelist.key().as_ref(),
            wallet.key().as_ref(),
        ],
        bump
    )]
    pub whitelist_data: Account<'info, WhitelistData>,
    pub system_program: Program<'info, System>,
}

// data structures
#[account]
pub struct Whitelist {
    pub authority: Pubkey
}


impl Whitelist {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBKEY_LENGTH;
}

#[account]
pub struct WhitelistData {
    pub whitelisted_data: Pubkey
}

impl WhitelistData {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBKEY_LENGTH;
}
