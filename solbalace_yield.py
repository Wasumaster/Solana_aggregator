import matplotlib.pyplot as plt
import numpy as np

# Projection horizon
years = np.arange(2023, 2031)

# Extrapolate missing market size data (in billions USD)
base_interest = np.array([0.8, 2.5, 3.8, 5.0, 6.2, 7.5, 9.0, 10.0])
if len(base_interest) < len(years):
    extra = len(years) - len(base_interest)
    growth = np.linspace(11, 14, extra)
    total_interest = np.concatenate([base_interest, growth])
else:
    total_interest = base_interest[:len(years)]

total_interest_usd = total_interest * 1e9

# Model assuming fixed 5% performance fee
profit = total_interest_usd * 0.05

plt.figure(figsize=(8, 5))
plt.plot(years, total_interest_usd, marker='o')
plt.title("Projected Solana Perpetuals Market Size")
plt.xlabel("Year")
plt.ylabel("USD")
plt.grid(True)
plt.tight_layout()
plt.show()

plt.figure(figsize=(8, 5))
plt.plot(years, profit, marker='o')
plt.title("Projected HanSOLo Profit (5% margin)")
plt.xlabel("Year")
plt.ylabel("USD")
plt.grid(True)
plt.tight_layout()
plt.show()