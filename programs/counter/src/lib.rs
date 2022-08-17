use anchor_lang::prelude::*;
// use anchor_lang::solana_program::account_info::AccountInfo;
use whitelisting::{
    self, 
    program::Whitelisting, 
    Whitelist,
    WhitelistData
};
use whitelisting::cpi::accounts::CreateWhitelist;
use anchor_lang::context::CpiContext;
use solana_program::{
    pubkey::Pubkey,
    pubkey
};

declare_id!("3pBmYTFPiUNstae4M2WUAQ6Giydr4nstQ6rnM1TXh8vk");
const USECASE: &str = "counter";

//Data logics
#[program]
pub mod counter {
    use super::*;

    pub fn create_counter(ctx: Context<CreateCounter>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.whitelist_config = ctx.accounts.whitelist_config.key();
        
        whitelisting::cpi::create_whitelist(ctx.accounts.create_whitelist_ctx(), USECASE.to_string())
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

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>) -> Result<()> {
        whitelisting::cpi::add_to_whitelist(ctx.accounts.add_to_whitelist_ctx())
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
    pub fn create_whitelist_ctx(&self) -> CpiContext<'_, '_, '_, 'info, CreateWhitelist<'info>> {
        let whitelisting_program_id = self.whitelisting_program.to_account_info();
        let whitelisting_accounts = CreateWhitelist {
            whitelist: self.whitelist_config.to_account_info(),
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
    #[account( 
        seeds = 
        [
            &USECASE.as_bytes(), 
            whitelisted_data.key().as_ref()
        ], 
        bump,
        seeds::program = pubkey!("DoXHuZ7cuGeDiLV6AwnoEsMLo1UGZ3AX5Mk7KgEy4UwV"),
        
    )]
    pub whitelist_config: Account<'info, Whitelist>,
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
    pub whitelisted_data: Signer<'info>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter: Account<'info, Counter>,
}

#[derive(Accounts)]
pub struct AddToWhitelist<'info> {
    pub authority: Signer<'info>,
    pub wallet: UncheckedAccount<'info>, 
    #[account(
        mut,
        seeds = 
        [
            &USECASE.as_bytes(), 
            authority.key().as_ref()
        ], 
        bump,
        seeds::program = pubkey!("DoXHuZ7cuGeDiLV6AwnoEsMLo1UGZ3AX5Mk7KgEy4UwV"), 
        has_one=authority
    )]
    pub whitelist_config: Account<'info, Whitelist>,
    #[account(mut)]
    /// CHECK: ok
    pub whitelist_data: UncheckedAccount<'info>,
    pub whitelisting_program: Program<'info, Whitelisting>,
    pub system_program: Program<'info, System>,
}

impl<'info> AddToWhitelist<'info> {
    pub fn add_to_whitelist_ctx(&self) -> CpiContext<'_, '_, '_, 'info, AddToWhitelist<'info>> {
        let whitelisting_program_id = self.whitelisting_program.to_account_info();
        let whitelist_data_accounts = AddToWhitelist {
            whitelist_config: self.whitelist_config,
            wallet: self.wallet,
            authority: self.authority,
            whitelist_data: self.whitelist_data,
            whitelisting_program: self.whitelisting_program,
            system_program: self.system_program,
        };
        CpiContext::new(whitelisting_program_id, whitelist_data_accounts)
    }
}

// data structures
#[account]
pub struct Counter {
    whitelist_config: Pubkey,
    pub count: u32,
}
