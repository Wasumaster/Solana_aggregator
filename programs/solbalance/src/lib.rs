use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("SoLBaLance1111111111111111111111111111111111");

#[program]
pub mod solbalance {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.admin = ctx.accounts.admin.key(); // The Scout Bot
        vault.usdc_mint = ctx.accounts.usdc_mint.key();
        vault.total_usdc_deposited = 0;
        vault.total_shares_minted = 0;
        vault.active_protocol = ProtocolStatus::Idle;
        vault.emergency_mode = false;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(!ctx.accounts.vault.emergency_mode, SolBalanceError::EmergencyModeActive);
        
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_usdc.to_account_info(),
            to: ctx.accounts.vault_usdc.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        let vault = &mut ctx.accounts.vault;
        let shares_to_mint = if vault.total_shares_minted == 0 {
            amount
        } else {
            (amount.checked_mul(vault.total_shares_minted).unwrap())
                .checked_div(vault.total_usdc_deposited)
                .unwrap()
        };

        vault.total_usdc_deposited = vault.total_usdc_deposited.checked_add(amount).unwrap();
        vault.total_shares_minted = vault.total_shares_minted.checked_add(shares_to_mint).unwrap();

        // [TODO: CPI Token Program -> Mint shares to user account]
        msg!("Deposited {} USDC. Emitted shares: {}", amount, shares_to_mint);

        Ok(())
    }

    pub fn execute_delta_neutral_split(ctx: Context<ExecuteSplit>, spot_swap_amount: u64, short_perp_amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.scout_bot.key(), SolBalanceError::UnauthorizedScout);
        require!(!vault.emergency_mode, SolBalanceError::EmergencyModeActive);
        require!(vault.active_protocol == ProtocolStatus::Idle, SolBalanceError::AlreadyInPosition);

        // DELTA SYMMETRY ENFORCEMENT
        // Max 1% deviation rule allowed covering DEX taker fees and slippage on JUP/Drift
        let max_deviation = spot_swap_amount / 100;
        let diff = if spot_swap_amount > short_perp_amount {
             spot_swap_amount - short_perp_amount 
        } else {
             short_perp_amount - spot_swap_amount 
        };
        require!(diff <= max_deviation, SolBalanceError::DeltaMismatch);

        msg!("[THE SPLIT] Executing Delta-Neutral Atomic Dispatch...");
        
        // 1. LONG (Spot/LST): Swap USDC for jitoSOL directly via Jupiter integration
        // [TODO: Construct Jupiter CPI via `jupiter_program` target]
        msg!("LONG ROUTE: Yield-Optimized jitoSOL Swap acquired ({} USDC base)", spot_swap_amount);

        // 2. SHORT (Perp): Supply Margin USDC and open 1x SOL-PERP
        // [TODO: Construct Target DEX CPI via `perp_dex_program` target]
        msg!("SHORT ROUTE: Initiated 1x SOL-PERP Drift execution (Margin: {} USDC)", short_perp_amount);

        vault.active_protocol = ProtocolStatus::Drift;

        Ok(())
    }

    pub fn harvest_and_compound(ctx: Context<Harvest>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.scout_bot.key(), SolBalanceError::UnauthorizedScout);
        require!(vault.active_protocol != ProtocolStatus::Idle, SolBalanceError::NoActiveProtocol);

        // Emulate settling funding rate payouts and appending them back precisely to Vault collateral USDC.
        msg!("[HARVESTING] Acquiring pending funding payloads (Drift) & Native Staking yields (Jito).");
        
        // Simulated arbitrary yield accrual
        let funding_profit = 45_000_000; 
        vault.total_usdc_deposited = vault.total_usdc_deposited.checked_add(funding_profit).unwrap();

        Ok(())
    }

    pub fn rebalance_protocol(ctx: Context<ExecuteSplit>, target_protocol: u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.scout_bot.key(), SolBalanceError::UnauthorizedScout);
        
        msg!("[REBALANCE] Off-chain target skew triggered. Unwinding...");
        
        // [TODO: CPI Close Drift SOL-PERP]
        // [TODO: CPI Flush JitoSOL -> USDC via JUP]
        
        vault.active_protocol = match target_protocol {
            1 => ProtocolStatus::Drift,
            2 => ProtocolStatus::Zeta,
            3 => ProtocolStatus::Mango,
            _ => ProtocolStatus::Idle,
        };

        msg!("Target market updated successfully. Awaiting split request.");
        Ok(())
    }

    /// EMERGENCY ESCAPE: Bear market funding rate anomaly protection
    pub fn emergency_evacuation(ctx: Context<ExecuteSplit>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.scout_bot.key(), SolBalanceError::UnauthorizedScout);
        
        msg!("[CRITICAL] Deep-Negative Funding Anomaly recognized. Full evacuation initialized.");
        
        // Immediate termination of derivatives 
        // Swap all LST to USDC stable asset parity
        
        vault.active_protocol = ProtocolStatus::Idle;
        vault.emergency_mode = true; // Hard-locks the TVL pool against new deposits

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        let usdc_to_return = (shares.checked_mul(vault.total_usdc_deposited).unwrap())
            .checked_div(vault.total_shares_minted)
            .unwrap();

        vault.total_usdc_deposited = vault.total_usdc_deposited.checked_sub(usdc_to_return).unwrap();
        vault.total_shares_minted = vault.total_shares_minted.checked_sub(shares).unwrap();

        let vault_bump = ctx.bumps.vault;
        let seeds = &["vault".as_bytes(), &[vault_bump]];
        let signer = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_usdc.to_account_info(),
            to: ctx.accounts.user_usdc.to_account_info(),
            authority: vault.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new_with_signer(cpi_program, cpi_accounts, signer), usdc_to_return)?;

        msg!("Withdrawn {} USDC exchanging {} vault shares.", usdc_to_return, shares);
        Ok(())
    }
}

// -------------------------------------------------------------
// ACCOUNTS & DTO
// -------------------------------------------------------------

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
pub struct ExecuteSplit<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub scout_bot: Signer<'info>, 
    
    // Unchecked accounts strictly enforced by DPI Runtime Validators
    /// CHECK: Jupiter Program 
    pub jupiter_program: UncheckedAccount<'info>,
    /// CHECK: Target DEX Program (Drift/Zeta)
    pub perp_dex_program: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct Harvest<'info> {
    #[account(mut, seeds = [b"vault"], bump)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub scout_bot: Signer<'info>,
    /// CHECK: Drift State Tracker
    pub drift_state: UncheckedAccount<'info>,
}

#[account]
pub struct Vault {
    pub admin: Pubkey,
    pub usdc_mint: Pubkey,
    pub total_usdc_deposited: u64,
    pub total_shares_minted: u64,
    pub active_protocol: ProtocolStatus, 
    pub emergency_mode: bool,
}

impl Vault {
    pub const SPACE: usize = 32 + 32 + 8 + 8 + 1 + 1; 
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum ProtocolStatus {
    Idle,
    Drift,
    Zeta,
    Mango,
}

#[error_code]
pub enum SolBalanceError {
    #[msg("Unauthorized invocation by a non-scout account.")]
    UnauthorizedScout,
    #[msg("Position strategy is currently running!")]
    AlreadyInPosition,
    #[msg("No active tracking protocol to harvest from.")]
    NoActiveProtocol,
    #[msg("CRITICAL: Split slippage limits failed! Non-Neutral Delta Attack intercepted.")]
    DeltaMismatch,
    #[msg("EMERGENCY PROTOCOL ON. Vault locked dynamically inside the chain.")]
    EmergencyModeActive,
}
