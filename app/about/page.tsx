import React from 'react';
import { Target, ShieldCheck, Cpu } from 'lucide-react';

export default function About() {
  return (
    <div className="container mx-auto max-w-3xl px-6 py-20">
      
      <div className="mb-16">
        <h1 className="text-4xl font-semibold tracking-tight text-white mb-6">
          About SolBalance
        </h1>
        <p className="text-neutral-400 text-lg leading-relaxed">
          SolBalance is a non-custodial yield aggregator built on Solana. It automates delta-neutral strategies, allowing users to harvest high APYs without taking directional market risks.
        </p>
      </div>

      <div className="space-y-12">
        <section className="border-l-2 border-neutral-800 pl-6">
          <div className="flex items-center gap-3 mb-4">
            <Cpu className="w-5 h-5 text-neutral-500" />
            <h2 className="text-xl font-medium text-neutral-200">The Delta-Neutral Strategy</h2>
          </div>
          <p className="text-neutral-500 leading-relaxed">
            By simultaneously holding a long spot position (such as Liquid Staked Tokens) and opening an equivalent short perpetual futures position, the portfolio effectively cancels out price volatility. The strategy purely aims to capture staking yields and short funding rates.
          </p>
        </section>

        <section className="border-l-2 border-neutral-800 pl-6">
          <div className="flex items-center gap-3 mb-4">
            <Target className="w-5 h-5 text-neutral-500" />
            <h2 className="text-xl font-medium text-neutral-200">Algorithmic Automation</h2>
          </div>
          <p className="text-neutral-500 leading-relaxed">
            Managing these positions manually requires 24/7 monitoring across multiple DEXs (like Drift, Zeta). Our on-chain programs and off-chain execution workers automatically rebalance collateral, preventing liquidations and optimizing basis yield at all times.
          </p>
        </section>

        <section className="border-l-2 border-neutral-800 pl-6">
          <div className="flex items-center gap-3 mb-4">
            <ShieldCheck className="w-5 h-5 text-neutral-500" />
            <h2 className="text-xl font-medium text-neutral-200">Risks & Transparency</h2>
          </div>
          <p className="text-neutral-500 leading-relaxed mb-4">
            While protected from asset price changes, delta-neutral strategies carry specific risks:
          </p>
          <ul className="list-disc list-inside text-neutral-500 space-y-2">
            <li><strong>Smart Contract Risk:</strong> Vulnerabilities within our vault or underlying DEXs.</li>
            <li><strong>Execution Risk:</strong> Periods of extremely high network congestion preventing timely rebalancing.</li>
            <li><strong>Funding Rate Volatility:</strong> Funding rates can occasionally turn negative (you pay to short), temporarily suppressing yields.</li>
          </ul>
        </section>
      </div>

    </div>
  );
}
