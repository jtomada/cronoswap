use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

declare_id!("H8LECSL9wE5HVVs1jAYoixq1MBsvoiRVrCvfiTbRA8Xo");


#[program]
pub mod cronos_dca {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        amount: f64,
        slippage: f64,
        cron: String,
    ) -> Result<()> {
        let dca = &mut ctx.accounts.dca_account;
        dca.input_mint = ctx.accounts.input_mint.key();
        dca.output_mint = ctx.accounts.output_mint.key();
        dca.amount = amount;
        dca.slippage = slippage;
        dca.cron = cron;
        msg!("initialized input mint, output mint, amount, slippage");

        Ok(())
    }

    pub fn swap(
        ctx: Context<Swap>,
    ) -> Result<()> {
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
    cron: String,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub dca_account: Account<'info, DcaConfig>,
    pub input_mint: Account<'info, Mint>,
    pub output_mint: Account<'info, Mint>,
    pub owner: Signer<'info>
}

#[derive(Accounts)]
pub struct Swap<'info> {
    pub dca_account: Account<'info, DcaConfig>,
    pub owner: Signer<'info>
}