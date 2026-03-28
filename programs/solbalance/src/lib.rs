use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("SoLBaLance1111111111111111111111111111111111");

#[program]
pub mod solbalance {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.admin = ctx.accounts.admin.key();
        vault.usdc_mint = ctx.accounts.usdc_mint.key();
        vault.total_usdc_deposited = 0;
        vault.total_shares_minted = 0;
        vault.strategy_active = false;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_usdc.to_account_info(),
            to: ctx.accounts.vault_usdc.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer(cpi_ctx, amount)?;

        let shares_to_mint = if vault.total_shares_minted == 0 {
            amount
        } else {
            (amount.checked_mul(vault.total_shares_minted).unwrap())
                .checked_div(vault.total_usdc_deposited)
                .unwrap()
        };

        vault.total_usdc_deposited = vault.total_usdc_deposited.checked_add(amount).unwrap();
        vault.total_shares_minted = vault.total_shares_minted.checked_add(shares_to_mint).unwrap();

        // TODO: CPI to token program to mint vault shares to user tracking account
        // ...

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;

        let usdc_to_return = (shares.checked_mul(vault.total_usdc_deposited).unwrap())
            .checked_div(vault.total_shares_minted)
            .unwrap();

        vault.total_usdc_deposited = vault.total_usdc_deposited.checked_sub(usdc_to_return).unwrap();
        vault.total_shares_minted = vault.total_shares_minted.checked_sub(shares).unwrap();

        // TODO: CPI to token program to burn user shares
        // ...

        let vault_bump = ctx.bumps.vault;
        let seeds = &["vault".as_bytes(), &[vault_bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_usdc.to_account_info(),
            to: ctx.accounts.user_usdc.to_account_info(),
            authority: vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);
        token::transfer(cpi_ctx, usdc_to_return)?;

        Ok(())
    }

    pub fn execute_strategy(ctx: Context<ExecuteStrategy>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.crank.key(), ErrorCode::Unauthorized);
        require!(!vault.strategy_active, ErrorCode::StrategyAlreadyActive);

        // TODO: CPI to Jupiter to swap half vault USDC to jitoSOL
        // ...

        // TODO: CPI to Drift to open 1x Short SOL-PERP with other half of USDC
        // ...

        vault.strategy_active = true;
        Ok(())
    }

    pub fn rebalance(ctx: Context<Rebalance>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.crank.key(), ErrorCode::Unauthorized);
        require!(vault.strategy_active, ErrorCode::StrategyInactive);

        // TODO: CPI to Drift to close Short SOL-PERP position and return collateral
        // ...

        // TODO: CPI to Jupiter to swap jitoSOL back to USDC
        // ...

        vault.strategy_active = false;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + Vault::SPACE,
        seeds = [b"vault"],
        bump
    )]
    pub vault: Account<'info, Vault>,
    pub usdc_mint: Account<'info, Mint>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,
    #[account(mut)]
    pub vault_usdc: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub vault_usdc: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_usdc: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct ExecuteStrategy<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub crank: Signer<'info>,
    /// CHECK: Jupiter Program
    pub jupiter_program: UncheckedAccount<'info>,
    /// CHECK: Drift Program
    pub drift_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Rebalance<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub crank: Signer<'info>,
    /// CHECK: Jupiter Program
    pub jupiter_program: UncheckedAccount<'info>,
    /// CHECK: Drift Program
    pub drift_program: UncheckedAccount<'info>,
}

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub usdc_mint: Pubkey,
    pub total_usdc_deposited: u64,
    pub total_shares_minted: u64,
    pub strategy_active: bool,
}

impl Vault {
    pub const SPACE: usize = 32 + 32 + 8 + 8 + 1;
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized crank/admin")]
    Unauthorized,
    #[msg("Strategy is already active")]
    StrategyAlreadyActive,
    #[msg("Strategy is inactive")]
    StrategyInactive,
}
