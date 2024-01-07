use anchor_lang::prelude::*;
use anchor_spl::token::{self, Burn, Mint, Token, TokenAccount};
declare_id!("2h7AsVek8XeCFPM6rJoskmM6KyLr39tQwkUFd8eb3TJM");

#[program]
mod immolate {
    use super::*;
    pub fn immolate(ctx: Context<Initialize>) -> Result<()> {
        // Get current balance
        let amount = ctx.accounts.from.amount;

        let cpi_accounts = Burn {
            mint: ctx.accounts.mint.to_account_info(),
            from: ctx.accounts.from.to_account_info(),
            authority: ctx.accounts.authority.to_account_info(),
        };
        ctx.accounts.mint.to_account_info();
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        // Issue burn instruction
        token::burn(cpi_ctx, amount)?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: This is the token to be burned
    #[account(mut)]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    /// CHECK: This is the token account that we want to burn tokens from
    #[account(mut)]
    pub from: Account<'info, TokenAccount>,
    /// CHECK: the authority of the token account
    pub authority: Signer<'info>,
}
