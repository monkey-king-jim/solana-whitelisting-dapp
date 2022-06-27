use anchor_lang::prelude::*;
// use anchor_lang::solana_program::account_info::AccountInfo;
// use anchor_spl::token::{self, Token, Mint, TokenAccount, Transfer};

declare_id!("8UCFsbJjuTzUimm4g9TuVooR3dKEC7MNV8wyqZp8TEKH");

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBKEY_LENGTH: usize = 32;
// const UNSIGNED64_LENGTH: usize = 8;

// business logic
#[program]
pub mod whitelisting {
    use super::*;

    pub fn create_whitelist(ctx: Context<CreateWhitelist>, added_address: u8) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.authority = ctx.accounts.user.key();
        counter_account.count = 0;
        Ok(())
    }
}

// data validators
#[derive(Accounts)]
#[instruction(added_address: u8)]
pub struct CreateWhitelist<'info> {
    #[account(
        init, 
        payer = user, 
        space = Whitelist::LEN, 
        seeds = [b"test", user.key().as_ref(), &[added_address]], 
        bump
    )]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// data structures
#[account]
pub struct Whitelist {
}


impl Whitelist {
    const LEN: usize = DISCRIMINATOR_LENGTH + PUBKEY_LENGTH;
}
