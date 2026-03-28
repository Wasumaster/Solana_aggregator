use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("SoLBaLance1111111111111111111111111111111111");

#[program]
pub mod solbalance {
    use super::*;

    /// Krok 1/Baza: Inicjalizacja skarbca (Vault)
    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        vault.admin = ctx.accounts.admin.key(); // Mózg algorytmu (Scout)
        vault.usdc_mint = ctx.accounts.usdc_mint.key();
        vault.total_usdc_deposited = 0;
        vault.total_shares_minted = 0;
        vault.active_protocol = ProtocolStatus::Idle;
        vault.emergency_mode = false;
        Ok(())
    }

    /// Krok 1 (The Scout): Depozyt od użytkownika
    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        require!(!ctx.accounts.vault.emergency_mode, SolBalanceError::EmergencyModeActive);
        
        // Przelew USDC od użytkownika do Skarbca (Vault)
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_usdc.to_account_info(),
            to: ctx.accounts.vault_usdc.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        token::transfer(CpiContext::new(cpi_program, cpi_accounts), amount)?;

        // Obliczenie udziałów (Shares) do wydania
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

        // [MOCK] CPI do Token Programu: Mint shares to user...
        msg!("Zdeponowano {} USDC. Udziały (shares) użytkownika: {}", amount, shares_to_mint);

        Ok(())
    }

    /// Krok 2 (The Split): Wejście w pozycję Delta-Neutral
    /// Rozdzielenie kapitału ułamkowego: np. zakup jitoSOL (Long) oraz 1x Short SOL-PERP (Drift)
    pub fn execute_delta_neutral_split(ctx: Context<ExecuteSplit>, spot_swap_amount: u64, short_perp_amount: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.scout_bot.key(), SolBalanceError::UnauthorizedScout);
        require!(!vault.emergency_mode, SolBalanceError::EmergencyModeActive);
        require!(vault.active_protocol == ProtocolStatus::Idle, SolBalanceError::AlreadyInPosition);

        // ZABEZPIECZENIE: Upewniamy się, że kwoty Spot i Short są symetryczne (Delta = 0).
        // Dla zachowania balansu, tolerujemy max 1% różnicy między dwiema nogami na pokrycie opłat (slippage + fees).
        let max_deviation = spot_swap_amount / 100;
        let diff = if spot_swap_amount > short_perp_amount {
             spot_swap_amount - short_perp_amount 
        } else {
             short_perp_amount - spot_swap_amount 
        };
        require!(diff <= max_deviation, SolBalanceError::DeltaMismatch);

        msg!("[THE SPLIT] Inicjowanie Atomowej Transakcji Delta-Neutral...");
        
        // 1. NOGA LONG (Spot): Przez Jupiter kupujemy jitoSOL za USDC. LST zoptymalizuje obrót.
        // [TODO: Skonstruuj Jupiter CPI wykorzystując konto `jupiter_program`]
        msg!("Noga LONG: Kupowanie jitoSOL ({} USDC) przez Jupiter", spot_swap_amount);

        // 2. NOGA SHORT (Perp): Tworzymy wpłatę Margin na Drifcie i otwieramy short SOL-PERP z lewarem 1x.
        // [TODO: Skonstruuj Drift CPI wykorzystując konto `drift_program`]
        msg!("Noga SHORT: Otwieranie 1x SOL-PERP na giełdzie Drift (Margin: {} USDC)", short_perp_amount);

        // Po pomyślnym zablokowaniu transakcji - zapisujemy stan do vault
        vault.active_protocol = ProtocolStatus::Drift;

        Ok(())
    }

    /// Krok 3 (The Harvesting): Pobieranie zapłaty od Longujących (Funding Rates)
    pub fn harvest_and_compound(ctx: Context<Harvest>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.scout_bot.key(), SolBalanceError::UnauthorizedScout);
        require!(vault.active_protocol != ProtocolStatus::Idle, SolBalanceError::NoActiveProtocol);

        // Odbieranie funding_rate payment. Na Drifcie polega to na `settle_funding_payment`.
        // Zyski są dodawane bezpośrednio z powrotem do Collaterala USDC na Drifcie,
        // powodując automatyczne zwiększenie wartości konta depozytariusza (Auto-Compounding).
        msg!("[THE HARVESTING] Przechwytywanie wpłat Funding Rates z Drifta i JitoSOL...");
        
        // Zmockowany zysk
        let funding_profit = 45_000_000; // 45 USDC
        vault.total_usdc_deposited = vault.total_usdc_deposited.checked_add(funding_profit).unwrap();

        Ok(())
    }

    /// Krok 4 (The Brain - Rebalancing): Zmiana protokołu (np. z Drifta na Zete) z powodu skoku APR
    pub fn rebalance_protocol(ctx: Context<ExecuteSplit>, target_protocol: u8) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.scout_bot.key(), SolBalanceError::UnauthorizedScout);
        
        msg!("[THE BRAIN] Wykryto wyższy Funding! Trwa ewakuacja z Drifta...");
        // Najpierw zamykamy obydwie pozycje (powrót do pełnego USDC)
        // [CPI DO DRIFTA ZAMYKAJĄCE SOL-PERP]
        // [CPI DO JUPITERA SPRZEDAJĄCE JitoSOL]
        
        // Następnie zmieniamy aktywny protokół
        vault.active_protocol = match target_protocol {
            1 => ProtocolStatus::Drift,
            2 => ProtocolStatus::Zeta,
            3 => ProtocolStatus::Mango,
            _ => ProtocolStatus::Idle,
        };

        // Krok otwarcia zlecenia na nowo musi zostać wykonany drugim splitem.
        msg!("Zaktualizowano docelowy rynek!");
        Ok(())
    }

    /// OCHRONA (Emergency): Ewakuacja kapitału w momencie powrotu rynku do Bessy i negatywnego fundingu.
    pub fn emergency_evacuation(ctx: Context<ExecuteSplit>) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        require!(vault.admin == ctx.accounts.scout_bot.key(), SolBalanceError::UnauthorizedScout);
        
        msg!("[EMERGENCY] Krytyczny negatywny Funding Rate! Zamykanie całego ekspozycji.");
        
        // Natychmiastowe rozwiązanie shorta na Drifcie/Zecie 
        // Wymiana JitoSOL na Jupierze spowrotem na Twarde USDC.
        
        vault.active_protocol = ProtocolStatus::Idle;
        vault.emergency_mode = true; // Zatrzymuje kolejne wpłaty do odwołania!

        Ok(())
    }

    /// Wypłata zysków użytkownika (Wyjście z systemu)
    pub fn withdraw(ctx: Context<Withdraw>, shares: u64) -> Result<()> {
        let vault = &mut ctx.accounts.vault;
        
        let usdc_to_return = (shares.checked_mul(vault.total_usdc_deposited).unwrap())
            .checked_div(vault.total_shares_minted)
            .unwrap();

        vault.total_usdc_deposited = vault.total_usdc_deposited.checked_sub(usdc_to_return).unwrap();
        vault.total_shares_minted = vault.total_shares_minted.checked_sub(shares).unwrap();

        // Powrót wyinkasowanych tokenów do użytkownika używając podpisu PDA (Seeds Vaultu).
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

        msg!("Zwrócono {} USDC za {} udziałów.", usdc_to_return, shares);
        Ok(())
    }
}

// -------------------------------------------------------------
// KONTEKSTY KONTA & STRUKTURY DANYCH
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
    pub admin: Signer<'info>, // Ten klucz zostanie zmapowany na Bot Executora
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
    pub scout_bot: Signer<'info>, // Maszyna nadzorująca system 
    
    // Potrzebujemy UncheckedAccounts dla zew. giełd (CPI Accounts Validation w runtime)
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
    /// CHECK: Drift State
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
    pub const SPACE: usize = 32 + 32 + 8 + 8 + 1 + 1; // Przestrzeń magazynowa bajtów
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
    #[msg("EMERGENCY PROTOCOL ACTIVE. Vault locked for deposits.")]
    EmergencyModeActive,
}
