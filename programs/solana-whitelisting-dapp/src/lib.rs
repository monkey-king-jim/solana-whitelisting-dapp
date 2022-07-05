use anchor_lang::prelude::*;
// use anchor_lang::solana_program::account_info::AccountInfo;
// use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};

declare_id!("GgcfkCKzrDHHnuhfAEvC7ohNjZk6hY9KZW2t9Sk7neRX");

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBKEY_LENGTH: usize = 32;
// const UNSIGNED64_LENGTH: usize = 8;

// business logics
#[program]
pub mod whitelisting {
    use super::*;

    pub fn create_whitelist(ctx: Context<CreateWhitelist>) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelist;
        whitelist.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>) -> Result<()> {
        Ok(())
    }
}

// data validators
#[derive(Accounts)]
pub struct CreateWhitelist<'info> {
    #[account(
        init, 
        payer = authority, 
        space = Whitelist::LEN, 
        seeds = [
            b"test", 
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
}

impl WhitelistData {
    const LEN: usize = DISCRIMINATOR_LENGTH;
}
