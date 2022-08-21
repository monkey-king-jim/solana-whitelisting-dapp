use anchor_lang::prelude::*;
// use anchor_lang::solana_program::account_info::AccountInfo;
use whitelisting::{
    self, 
    program::Whitelisting, 
    WhitelistConfig,
    WhitelistData
};
use whitelisting::cpi::accounts::CreateWhitelistConfig;
use anchor_lang::context::CpiContext;
use solana_program::{
    pubkey::Pubkey,
    pubkey
};

declare_id!("3pBmYTFPiUNstae4M2WUAQ6Giydr4nstQ6rnM1TXh8vk");
// the use case seed for this counter program is "counter"
const USECASE: &str = "counter";

// business logics
#[program]
pub mod counter {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.whitelist_config = ctx.accounts.whitelist_config.key();
        
        // make a cpi to create a whitelist config account along with the counter account
        whitelisting::cpi::create_whitelist_config(ctx.accounts.create_whitelist_config_ctx(), USECASE.to_string())
    }

    pub fn update(ctx: Context<Update>, count: u32) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        counter_account.count = count;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        counter_account.count = counter_account.count.checked_add(1).unwrap();
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter;
        if counter_account.count > 0 {
            counter_account.count -= 1;
        }
        Ok(())
    }
}

// data validators
#[derive(Accounts)]
pub struct CreateCounter<'info> {
    // space: 32 public key (whitelist base program) + 8 discrimator + 4 count size + 1 bump
    #[account(
        init, 
        payer = user, 
        space = 32 + 8 + 4 + 1, 
        seeds = [user.key().as_ref()], 
        bump
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    /// CHECK: ok
    pub whitelist_config: UncheckedAccount<'info>,
    pub whitelisting_program: Program<'info, Whitelisting>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateCounter<'info> {
    pub fn create_whitelist_config_ctx(&self) -> CpiContext<'_, '_, '_, 'info, CreateWhitelistConfig<'info>> {
        let whitelisting_program_id = self.whitelisting_program.to_account_info();
        let whitelisting_accounts = CreateWhitelistConfig {
            whitelist_config: self.whitelist_config.to_account_info(),
            authority: self.user.to_account_info(),
            system_program: self.system_program.to_account_info(),
        };
        CpiContext::new(whitelisting_program_id, whitelisting_accounts)
    }
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
    // verify the whitelist config account passed in validator is the one associated with this counter
    #[account( 
        seeds = 
        [
            &USECASE.as_bytes(), 
            whitelist_config.authority.key().as_ref()
        ], 
        bump,
        seeds::program = pubkey!("DoXHuZ7cuGeDiLV6AwnoEsMLo1UGZ3AX5Mk7KgEy4UwV"),
        
    )]
    pub whitelist_config: Account<'info, WhitelistConfig>,
    // verify if the caller was whitelisted; if false, the caller cannot make change to the counter
    #[account(
        seeds = 
        [
            whitelist_config.authority.key().as_ref(), 
            whitelisted_data.key().as_ref()
        ], 
        bump,
        seeds::program = whitelist_config.key(),
        has_one = whitelisted_data
    )]
    pub whitelist_data: Account<'info, WhitelistData>,
    #[account(mut)]
    pub whitelisted_data: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

// data structures
#[account]
pub struct Counter {
    whitelist_config: Pubkey,
    pub count: u32,
}
