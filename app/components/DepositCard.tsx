'use client';

import React, { useState } from 'react';
import { useWallet } from '@solana/wallet-adapter-react';

export default function DepositCard() {
  const { connected } = useWallet();
  const [amount, setAmount] = useState<string>('');

  const handleDeposit = async () => {
    if (!amount || !connected) return;
    console.log(`Initiating Deposit CPI for ${amount} USDC`);

  };

  const handleWithdraw = async () => {
    if (!amount || !connected) return;
    console.log(`Initiating Withdraw CPI for ${amount} Shares`);

  };

  return (
    <div className="border border-neutral-800/60 bg-[#0a0a0a] p-6 lg:p-8 rounded-xl relative overflow-hidden backdrop-blur-md">
      
      {/* Decorative gradient flare - extremely subtle */}
      <div className="absolute top-0 right-0 w-64 h-64 bg-emerald-500/5 blur-[100px] rounded-full pointer-events-none"></div>
      
      <div className="flex justify-between items-center mb-8">
        <h3 className="text-sm font-medium text-neutral-100 tracking-wide uppercase">Vault Access</h3>
        <div className="flex items-center gap-2">
          <span className="relative flex h-2 w-2">
            {connected && <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-emerald-400 opacity-20"></span>}
            <span className={`relative inline-flex rounded-full h-2 w-2 ${connected ? 'bg-emerald-500' : 'bg-neutral-600'}`}></span>
          </span>
          <span className="text-neutral-500 text-xs font-medium uppercase tracking-wider">{connected ? 'Active' : 'Offline'}</span>
        </div>
      </div>
      
      <div className="mb-8">
        <label className="block text-[11px] font-medium uppercase text-neutral-500 mb-2 tracking-wider">Amount (USDC)</label>
        <div className="flex bg-neutral-950/50 border border-neutral-800 rounded-lg p-1.5 focus-within:border-neutral-600 focus-within:bg-neutral-900 transition-all duration-300">
          <input
            type="number"
            value={amount}
            onChange={(e) => setAmount(e.target.value)}
            className="bg-transparent w-full text-neutral-200 p-2.5 outline-none font-mono text-base placeholder:text-neutral-700"
            placeholder="0.00"
          />
          <button className="px-4 font-medium text-neutral-400 text-xs hover:text-neutral-200 hover:bg-neutral-800/50 rounded-md transition-colors">
            MAX
          </button>
        </div>
      </div>

      <div className="grid grid-cols-2 gap-3">
        <button
          onClick={handleDeposit}
          disabled={!connected || !amount}
          className="bg-neutral-100 hover:bg-white text-neutral-950 font-semibold tracking-wide py-3.5 px-4 rounded-lg transition-all duration-300 disabled:opacity-40 disabled:cursor-not-allowed text-sm"
        >
          {connected ? 'Deposit' : 'Connect Wallet'}
        </button>
        <button
          onClick={handleWithdraw}
          disabled={!connected || !amount}
          className="bg-transparent border border-neutral-800 text-neutral-300 hover:bg-neutral-900 font-medium tracking-wide py-3.5 px-4 rounded-lg transition-all duration-300 disabled:opacity-40 disabled:cursor-not-allowed text-sm hover:border-neutral-700"
        >
          Withdraw
        </button>
      </div>

      <div className="mt-6 flex justify-between items-center text-xs font-medium text-neutral-500 pt-6 border-t border-neutral-800/50">
        <span>Available Balance</span>
        <span className="text-neutral-300 font-mono">0.00 USDC</span>
      </div>
    </div>
  );
}
