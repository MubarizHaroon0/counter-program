use anchor_lang::prelude::*;

declare_id!("xB4KEJykRgdqj6M3FopufhvmCHpta9NS5AxWu6RhEeE");

#[program]
pub mod counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = 0;
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter += 1;
        msg!("Incremented counter to {}", counter_account.counter);
        Ok(())
    }

    pub fn decrement(ctx: Context<Decrement>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = counter_account.counter.saturating_sub(1);
        msg!("Decremented counter to {}", counter_account.counter);
        Ok(())
    }

    pub fn reset(ctx: Context<Reset>) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = 0;
        msg!("Reset counter to {}", counter_account.counter);
        Ok(())
    }

    pub fn update(ctx: Context<Update>, value: u32) -> Result<()> {
        let counter_account = &mut ctx.accounts.counter_account;
        counter_account.counter = value;
        msg!("Updated counter to {}", counter_account.counter);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 4)]
    pub counter_account: Account<'info, CounterAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[derive(Accounts)]
pub struct Decrement<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[derive(Accounts)]
pub struct Reset<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub counter_account: Account<'info, CounterAccount>,
}

#[account]
pub struct CounterAccount {
    pub counter: u32,
}
