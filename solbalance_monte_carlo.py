import numpy as np
import matplotlib.pyplot as plt
import math
from dataclasses import dataclass

# ==========================================
# 🪐 SOLBALANCE - THE DARK FOREST SIMULATOR v3.0
# (Uncensored Reality / Extreme Stress Test)
# ==========================================

@dataclass
class RealityConfig:
    num_simulations: int = 1000
    num_days: int = 365
    initial_capital: float = 10000.0
    
    # Asset parameters (Tamed down base drift)
    initial_sol_price: float = 150.0
    sol_daily_volatility: float = 0.05
    sol_daily_drift: float = 0.0001
    
    # MERTON JUMP DIFFUSION (Black Swans / Crashes)
    jump_intensity: float = 3.0      # Average 3 major jumps per year
    jump_mean: float = -0.05         # Jumps skew negative (fear is faster than greed)
    jump_std: float = 0.12           # High variance in crash severity
    
    # Yield & Hidden Costs
    base_jitosol_apy: float = 0.07   # Base APY, but will fluctuate
    borrow_apr: float = 0.03         # 3% hidden holding/borrow cost on Drift
    
    # Funding Rate (Ornstein-Uhlenbeck)
    funding_theta: float = 0.15
    funding_mu: float = 0.01         # Long-term mean is barely positive (1% APR)
    funding_sigma: float = 0.04
    
    # Harsh Realities (Slippage & Infrastructure)
    base_fee: float = 0.001          # Standard 0.1% fee
    rpc_base_fail_rate: float = 0.05 # 5% base chance bot fails to send TX
    
    # Risk Engine
    rebalance_trigger_pct: float = 0.40
    liquidation_threshold: float = 0.0
    emergency_funding_threshold: float = -0.20
    lst_depeg_shock: float = 0.04    # 4% depeg during massive drops

class DarkForestSimulation:
    def __init__(self, config: RealityConfig):
        self.cfg = config
        np.random.seed(6) # The number of the beast for a beastly market

    def _calculate_exponential_slippage(self, actual_return: float) -> float:
        """Slippage explodes exponentially during high volatility."""
        return self.cfg.base_fee + (math.exp(abs(actual_return) * 12) - 1) * 0.005

    def _check_rpc_failure(self, actual_return: float) -> bool:
        """Simulates blockchain congestion. Higher volatility = higher chance of bot failure."""
        volatility_penalty = abs(actual_return) * 2.0
        fail_chance = self.cfg.rpc_base_fail_rate + volatility_penalty
        return np.random.random() < fail_chance

    def run(self):
        final_portfolios = []
        total_liquidations = 0
        total_emergencies = 0
        total_rebalances = 0
        rpc_failures_count = 0
        portfolio_paths = []
        
        print("="*70)
        print("💀 WELCOME TO THE DARK FOREST: Initializing Brutal Reality Quant Engine...")
        print(f"Executing {self.cfg.num_simulations:,.0f} Parallel Universes (Merton Jump-Diffusion enabled)")
        print("="*70)

        for sim in range(self.cfg.num_simulations):
            capital = self.cfg.initial_capital
            sol_price = self.cfg.initial_sol_price
            
            entry_slippage = self._calculate_exponential_slippage(self.cfg.sol_daily_volatility)
            
            # Allocation
            jito_sol_amount = (capital / 2 * (1 - entry_slippage)) / sol_price
            drift_margin_usdc = capital / 2 * (1 - entry_slippage)
            short_size_sol = drift_margin_usdc / sol_price 
            
            daily_portfolio_value = np.zeros(self.cfg.num_days)
            current_funding_rate = self.cfg.funding_mu
            
            is_emergency = False
            emergency_cooldown = 0
            depeg_cooldown = 0
            
            for day in range(self.cfg.num_days):
                if is_emergency:
                    daily_portfolio_value[day] = capital
                    emergency_cooldown -= 1
                    if emergency_cooldown <= 0:
                        is_emergency = False
                        entry_slippage = self._calculate_exponential_slippage(self.cfg.sol_daily_volatility)
                        jito_sol_amount = (capital / 2 * (1 - entry_slippage)) / sol_price
                        drift_margin_usdc = capital / 2 * (1 - entry_slippage)
                        short_size_sol = drift_margin_usdc / sol_price
                    continue

                # 1. MERTON JUMP DIFFUSION PRICE ACTION
                z1 = np.random.normal(0, 1)
                gbm_return = self.cfg.sol_daily_drift + self.cfg.sol_daily_volatility * z1
                
                # Poisson Jumps (Sudden crashes/pumps)
                n_jumps = np.random.poisson(self.cfg.jump_intensity / 365)
                jump_multiplier = 1.0
                for _ in range(n_jumps):
                    jump_multiplier *= np.random.lognormal(self.cfg.jump_mean, self.cfg.jump_std)
                
                prev_sol_price = sol_price
                sol_price = sol_price * (1 + gbm_return) * jump_multiplier
                actual_daily_return = (sol_price / prev_sol_price) - 1
                
                # Funding Rate (OU Process)
                dW = np.random.normal(0, 1)
                current_funding_rate += self.cfg.funding_theta * (self.cfg.funding_mu - current_funding_rate) + self.cfg.funding_sigma * dW
                
                # 2. DYNAMIC DE-PEG & YIELD
                lst_discount = 0.0
                if actual_daily_return < -0.10: 
                    depeg_cooldown = 3 
                if depeg_cooldown > 0:
                    lst_discount = self.cfg.lst_depeg_shock
                    depeg_cooldown -= 1

                # Dynamic APY: Drops in bear markets, rises in bull
                dynamic_jito_apy = max(0.02, self.cfg.base_jitosol_apy + (actual_daily_return * 0.1))
                jito_sol_amount *= (1 + (dynamic_jito_apy / 365))
                jito_sol_usdc_value = jito_sol_amount * sol_price * (1 - lst_discount)
                
                # 3. MARK TO MARKET & HIDDEN COSTS
                daily_short_pnl = short_size_sol * (prev_sol_price - sol_price)
                drift_margin_usdc += daily_short_pnl
                
                notional_short_value = short_size_sol * sol_price
                # Add funding rate and subtract borrow costs
                daily_cashflow = notional_short_value * ((current_funding_rate - self.cfg.borrow_apr) / 365)
                drift_margin_usdc += daily_cashflow

                # 4. BRUTAL RISK ENGINE
                # A) Keepers Liquidate You (Instant, no RPC needed on your end)
                if drift_margin_usdc <= self.cfg.liquidation_threshold:
                    total_liquidations += 1
                    slippage = self._calculate_exponential_slippage(actual_daily_return)
                    capital = jito_sol_usdc_value * (1 - slippage) # Salvage remaining LST
                    is_emergency = True
                    emergency_cooldown = 14
                    daily_portfolio_value[day] = capital
                    break

                # B) Auto-Rebalance Attempt
                short_loss_ratio = drift_margin_usdc / notional_short_value
                if short_loss_ratio < self.cfg.rebalance_trigger_pct:
                    # Check if Solana RPC is congested during this dump
                    if self._check_rpc_failure(actual_daily_return):
                        rpc_failures_count += 1
                        # Rebalance fails! Margin is bleeding, must wait for next day.
                    else:
                        total_rebalances += 1
                        jito_sell_amount = jito_sol_amount * 0.25
                        jito_sol_amount -= jito_sell_amount
                        
                        slippage = self._calculate_exponential_slippage(actual_daily_return)
                        rebalance_gas_usdc = jito_sell_amount * sol_price * (1 - lst_discount)
                        fees = rebalance_gas_usdc * slippage * 2 # Pay for Jup + Drift
                        
                        drift_margin_usdc += (rebalance_gas_usdc - fees)
                        
                        total_equity = (jito_sol_amount * sol_price) + drift_margin_usdc
                        short_size_sol = (total_equity / 2) / sol_price

                # C) Toxic Funding Exit Attempt
                if current_funding_rate < self.cfg.emergency_funding_threshold:
                    if self._check_rpc_failure(actual_daily_return):
                        rpc_failures_count += 1
                    else:
                        total_emergencies += 1
                        slippage = self._calculate_exponential_slippage(actual_daily_return)
                        
                        capital = (jito_sol_usdc_value * (1 - slippage)) + (drift_margin_usdc * (1 - slippage))
                        is_emergency = True
                        emergency_cooldown = 10
                        jito_sol_amount = 0
                        drift_margin_usdc = 0

                daily_portfolio_value[day] = (jito_sol_usdc_value + drift_margin_usdc) if not is_emergency else capital

            if not is_emergency and drift_margin_usdc > 0:
                final_portfolios.append(jito_sol_amount * sol_price + drift_margin_usdc)
            else:
                final_portfolios.append(capital)
                
            if sim < 60:
                portfolio_paths.append(daily_portfolio_value)

        self._print_report(np.array(final_portfolios), total_liquidations, total_rebalances, total_emergencies, rpc_failures_count)
        self._render_charts(portfolio_paths, np.array(final_portfolios))

    def _print_report(self, portfolios, liquidations, rebalances, emergencies, rpc_fails):
        avg_final = np.mean(portfolios)
        avg_apy = ((avg_final - self.cfg.initial_capital) / self.cfg.initial_capital) * 100
        win_rate = np.sum(portfolios > self.cfg.initial_capital) / self.cfg.num_simulations * 100

        print(f"\n✅ SIMULATION COMPLETE. Surviving the Dark Forest.")
        print(f"\n--- HARSH REALITY METRICS ---")
        print(f"💰 Initial Deposit:       ${self.cfg.initial_capital:,.2f}")
        print(f"📉 Avg Net Capital:       ${avg_final:,.2f} (Reality Check)")
        print(f"📊 Expected Real APY:     {avg_apy:.2f}%")
        print(f"\n--- CARNAGE REPORT ---")
        print(f"💀 Total Liquidations:    {liquidations} (Keepers got you)")
        print(f"🔌 RPC Node Failures:     {rpc_fails} times bot choked during panic")
        print(f"🛡️ Successful Rescues:    {rebalances} rebalances executed")
        print(f"🏃 Emergency Exits:       {emergencies}")
        print(f"🏆 Survival Rate (>0%):   {win_rate:.2f}%\n")

    def _render_charts(self, paths, final_portfolios):
        plt.style.use('dark_background')
        fig = plt.figure(figsize=(15, 6))

        ax1 = plt.subplot(121)
        for path in paths:
            # Color code by survival
            color = '#ef4444' if path[-1] < self.cfg.initial_capital else '#10b981'
            ax1.plot(path, alpha=0.25, linewidth=1.2, color=color)
        ax1.axhline(self.cfg.initial_capital, color='#ffffff', linestyle='--', alpha=0.5)
        ax1.set_title("Vault Value: Blood in the Streets (Merton Jumps & RPC Fails)", color="white")
        ax1.set_xlabel("Days")
        ax1.set_ylabel("Vault Net Value (USDC)")
        ax1.grid(color='#262626', linestyle=':')

        ax2 = plt.subplot(122)
        avg_apy = ((np.mean(final_portfolios) - self.cfg.initial_capital) / self.cfg.initial_capital) * 100
        ax2.hist(final_portfolios, bins=50, color='#6366f1', edgecolor='black', alpha=0.85)
        ax2.axvline(self.cfg.initial_capital, color='#ef4444', linestyle='dashed', linewidth=2)
        ax2.axvline(np.mean(final_portfolios), color='#fbbf24', linestyle='solid', linewidth=2, label=f'True Mean APY: {avg_apy:.1f}%')
        ax2.set_title("Probability Distribution (Fat Tails Enabled)", color="white")
        ax2.set_xlabel("Final Value (USDC)")
        ax2.legend()
        ax2.grid(color='#262626', linestyle=':')

        plt.tight_layout()
        plt.show()

if __name__ == "__main__":
    config = RealityConfig()
    sim = DarkForestSimulation(config)
    sim.run()