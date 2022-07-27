use anchor_lang::prelude::*;
// use anchor_lang::solana_program::account_info::AccountInfo;
use whitelisting::{self, program::Whitelisting, Whitelist};
use whitelisting::cpi::accounts::CreateWhitelist;
use anchor_lang::context::CpiContext;

declare_id!("8UCFsbJjuTzUimm4g9TuVooR3dKEC7MNV8wyqZp8TEKH");


//Data logics
#[program]
pub mod counter {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.whitelist = ctx.accounts.whitelisting_program.key();
        let key = "counter";
        whitelisting::cpi::create_whitelist(ctx.accounts.create_whitelist_ctx(), key.to_string())
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
    pub whitelist: UncheckedAccount<'info>,
    pub whitelisting_program: Program<'info, Whitelisting>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateCounter<'info> {
    pub fn create_whitelist_ctx(&self) -> CpiContext<'_, '_, '_, 'info, CreateWhitelist<'info>> {
        let whitelisting_program_id = self.whitelisting_program.to_account_info();
        let whitelisting_accounts = CreateWhitelist {
            whitelist: self.whitelist.to_account_info(),
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
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}


// data structures
#[account]
pub struct Counter {
    whitelist: Pubkey,
    pub count: u32,
}