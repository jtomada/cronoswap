use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod cronos_dca {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        amount: f64,
        slippage: f64,
    ) -> Result<()> {
        ctx.accounts.dca_account.input_mint = ctx.accounts.input_mint.key();
        ctx.accounts.dca_account.output_mint = ctx.accounts.output_mint.key();
        ctx.accounts.dca_account.amount = amount;
        ctx.accounts.dca_account.slippage = slippage;
        msg!("initialized input mint, output mint, amount, slippage");
        Ok(())
    }
}

#[account]
#[derive(Default)]
pub struct DcaConfig {
    input_mint: Pubkey,
    output_mint: Pubkey,
    amount: f64,
    slippage: f64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub dca_account: Account<'info, DcaConfig>,
    pub input_mint: Account<'info, Mint>,
    pub output_mint: Account<'info, Mint>,
    pub owner: Signer<'info>
}
