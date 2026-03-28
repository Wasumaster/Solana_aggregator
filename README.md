# 🪐 SolBalance: Delta-Neutral Yield Harvester

![Solana](https://img.shields.io/badge/Solana-362D59?style=for-the-badge&logo=solana&logoColor=white)
![Next.js](https://img.shields.io/badge/next.js-%23000000.svg?style=for-the-badge&logo=next.js&logoColor=white)
![TailwindCSS](https://img.shields.io/badge/tailwindcss-%2338B2AC.svg?style=for-the-badge&logo=tailwind-css&logoColor=white)
![Anchor](https://img.shields.io/badge/Anchor-Rust-E34234?style=for-the-badge&logo=rust&logoColor=white)

**SolBalance** is a non-custodial, delta-neutral yield aggregation protocol built natively for the Solana ecosystem. By simultaneously holding long spot positions (via Liquid Staking Tokens like JitoSOL) and executing equivalent short perpetual futures positions (on DEXs like Drift and Zeta), the algorithmic vault guarantees absolute price neutrality.

The protocol purely harvests LST basis yields and short funding rates across multiple Solana perp markets simultaneously—fully automated.

---

## 🚀 Key Features

* **Algorithmic Execution (Worker):** Off-chain TypeScript execution bots autonomously fetch market skews to rebalance the portfolio constantly via on-chain smart contracts.
* **Premium Dashboard Interface:** Built using Next.js 16 App Router and TailwindCSS v4. It features a muted, futuristic "Premium Tech" UI equipped with interactive Recharts, live data feeds, and fully integrated wallet connection endpoints.
* **Live Solana Price Feed:** Integrates real-time SOL/USD pricing directly into the dashboard header.
* **Market Skew & Funding Rate Analysis:** Live simulation/visualizations showing protocol exposure across various DEXs (Drift, Zeta, Jupiter, Flash).
* **Automated Activity Log:** Track precisely when background automated rebalancing and yield harvesting occur right on the front-end dashboard.

---

## 🏗️ Architecture & Project Structure

The repository is structured as a complete full-stack environment.

```
ArgeSolans/
├── app/                      # Next.js 16 (App Router) Frontend
│   ├── about/                # Education & Mechanism breakdown page
│   ├── components/           # UI Components (DepositCard, Navbar, WalletProvider)
│   ├── dashboard/            # Core dApp Interface (Charts, KPIs, API fetching)
│   ├── globals.css           # Global Tailwind CSS definitions (v4)
│   ├── layout.tsx            # Root HTML layout with Wallet Context
│   └── page.tsx              # Minimalist Landing Page (Marketing site)
├── programs/                 # Solana On-Chain Programs
│   └── solbalance/           # Anchor Framework Smart Contracts (Rust)
├── worker/                   # Off-chain execution logic (TypeScript)
│   └── index.ts              # Bots listening to Funding Rates & triggering rebalances
├── .gitignore                
├── package.json              # Node dependencies configuration
├── postcss.config.mjs        # PostCSS (bridge for TailwindCSS v4)
└── tsconfig.json             # TypeScript configuration
```

---

## 🛠️ Technology Stack

### Frontend (Client)
- **Framework:** Next.js (Version 16 - App Router)
- **Styling:** Tailwind CSS (Version 4.2+ via PostCSS)
- **UI & Icons:** Lucide-React
- **Visualization:** Recharts (Interactive Line/Area projections)
- **Web3 Ecosystem:** `@solana/wallet-adapter-react`, `@solana/web3.js`

### Backend (Smart Contracts & Workers)
- **On-Chain:** Rust x Anchor Framework (`programs/`)
- **Off-Chain Engine:** Node.js TypeScript execution bots (`worker/`)

---

## ⚙️ Setup & Installation

Follow these steps to spin up the entire environment locally.

### 1. Requirements
- Node.js (v18+)
- Rust & Cargo
- Solana CLI
- Anchor CLI

### 2. Clone & Install
Install the Node dependencies containing both the frontend and worker environment modules.
```bash
# Install NPM dependencies
npm install

# (Optional) Verify Tailwind CSS modules for v4
npm install @tailwindcss/postcss --legacy-peer-deps
```

### 3. Running the Frontend (Next.js Application)
Start the high-performance local development server.
```bash
npm run dev
```
Navigate to `http://localhost:3000` in your web browser. 

* **To see the landing page:** Use the root path (`/`).
* **To open the Wallet & Analytical Dashboard:** Click "Launch App" or navigate directly to `http://localhost:3000/dashboard`.

### 4. Running the Smart Contracts (Anchor)
Compile the vault program using Anchor. Make sure you have your local Solana test validator running.
```bash
cd programs/solbalance
anchor build
anchor test
```

### 5. Starting the Execution Bots (Worker)
Start the TypeScript off-chain algorithmic worker to monitor simulated or live market shifts and rebalance the portfolio.
```bash
cd worker
npm run start # Adjust based on your TS execution environment
```

---

## ⚠️ Disclaimer & Risk Analysis

The SolBalance smart contracts and automated strategies are highly experimental. While the architecture intends to prevent crypto-asset price exposure (Delta = 0), DeFi mechanisms still pose risks:
- **Smart Contract Vulnerability Risk:** Exploitation within internal code or composed protocols.
- **Execution & Liquidation Risk:** Inability of off-chain workers to process rebalancing transactions during times of severe network congestion on Solana.
- **Negative Funding Risk:** While short positions usually receive funding during bull periods, extreme bearish shifts could cause shorts to pay longs. 

*Not financial advice. Use on testnet and devnet initially to simulate outcomes.*

---
**Crafted with 🖤 for the Solana Ecosystem.**
