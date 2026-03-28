import React from 'react';
import Link from 'next/link';
import { Shield, Zap, TrendingUp, ChevronRight } from 'lucide-react';

export default function Home() {
  return (
    <main className="min-h-screen">
      <div className="container mx-auto max-w-5xl px-6 py-24 md:py-32 flex flex-col items-center text-center">
        
        <div className="inline-flex items-center gap-2 px-4 py-2 rounded-full border border-neutral-800 bg-neutral-900/30 text-sm font-medium text-neutral-400 mb-8">
          <span className="w-2 h-2 rounded-full bg-emerald-500/80 animate-pulse"></span>
          Live Execution Engine
        </div>
        
        <h1 className="text-5xl md:text-7xl font-semibold tracking-tight text-white leading-tight mb-8">
          Delta-neutral <br/> yield harvester.
        </h1>
        
        <p className="text-neutral-500 text-lg md:text-xl leading-relaxed max-w-2xl mb-12">
          Deposit USDC. Automatically earn LST yield while capturing short funding rates across Solana perpetual DEXs. Powered by algorithm-driven execution.
        </p>

        <div className="flex flex-col sm:flex-row gap-4 w-full sm:w-auto">
          <Link href="/dashboard" className="bg-neutral-100 hover:bg-white text-neutral-950 font-semibold text-lg px-8 py-4 rounded-lg transition-transform hover:scale-[1.02] flex items-center justify-center gap-2">
            Launch App <ChevronRight className="w-5 h-5 text-neutral-500" />
          </Link>
          <Link href="/about" className="bg-transparent border border-neutral-800 hover:bg-neutral-900 text-neutral-300 font-semibold text-lg px-8 py-4 rounded-lg transition-all flex items-center justify-center">
            How it works
          </Link>
        </div>
        
      </div>

      {/* Feature Section */}
      <div className="border-t border-neutral-900/80 bg-[#0a0a0a]">
        <div className="container mx-auto max-w-5xl px-6 py-24 grid grid-cols-1 md:grid-cols-3 gap-12">
          
          <div className="flex flex-col items-start text-left">
            <div className="w-12 h-12 rounded-lg border border-neutral-800 bg-neutral-900/50 flex items-center justify-center mb-6">
              <Zap className="text-emerald-500 w-6 h-6" />
            </div>
            <h3 className="text-xl font-medium text-neutral-200 mb-3">Algorithmic Execution</h3>
            <p className="text-neutral-500 leading-relaxed text-sm">
              Our smart contracts automatically rebalance positions between Spot and Perp markets to maintain neutral exposure.
            </p>
          </div>

          <div className="flex flex-col items-start text-left">
            <div className="w-12 h-12 rounded-lg border border-neutral-800 bg-neutral-900/50 flex items-center justify-center mb-6">
              <TrendingUp className="text-emerald-500 w-6 h-6" />
            </div>
            <h3 className="text-xl font-medium text-neutral-200 mb-3">Compounding APY</h3>
            <p className="text-neutral-500 leading-relaxed text-sm">
              Basis yield and LST staking rewards are constantly auto-compounded, maximizing your capital efficiency.
            </p>
          </div>

          <div className="flex flex-col items-start text-left">
            <div className="w-12 h-12 rounded-lg border border-neutral-800 bg-neutral-900/50 flex items-center justify-center mb-6">
              <Shield className="text-emerald-500 w-6 h-6" />
            </div>
            <h3 className="text-xl font-medium text-neutral-200 mb-3">Non-Custodial</h3>
            <p className="text-neutral-500 leading-relaxed text-sm">
              SolBalance acts as an execution router. All assets are held natively on Solana DEXs directly via your designated vault.
            </p>
          </div>

        </div>
      </div>
    </main>
  );
}
