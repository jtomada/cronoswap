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

// quote amount comes from constant product curve
// https://web.stanford.edu/~guillean/papers/uniswap_analysis.pdf
fn div_ceiling(numerator: u128, denominator: u128) -> u128 {
    if numerator % denominator == 0 { 
        numerator / denominator 
    } else { 
        numerator / denominator + 1 
    }
}

fn get_constant_product_output_amount(
    input_pool_balance: u64,
    output_pool_balance: u64,
    input_amount: u64,
) -> u64 {
    let rsrv_b: u128 = From::from(input_pool_balance);
    let rsrv_a: u128 = From::from(output_pool_balance);
    let delta_b: u128 = From::from(input_amount);

    let orca_fee = (30, 10000);

    let fee_delta_b = div_ceiling(delta_b * orca_fee.0, orca_fee.1);
    let delta_b_postfee = delta_b - fee_delta_b;

    let k = rsrv_b * rsrv_a;
    let rsrv_b_posttrade = rsrv_b + delta_b_postfee;
    let rsrv_a_posttrade = div_ceiling(k, rsrv_b_posttrade);

    let delta_a = rsrv_a - rsrv_a_posttrade;

    let expected_oa_u64: u64 = TryFrom::try_from(delta_a).unwrap();
    expected_oa_u64
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