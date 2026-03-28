'use client';

import React from 'react';
import Link from 'next/link';
import { usePathname } from 'next/navigation';
import { Activity } from 'lucide-react';
import { WalletMultiButton } from '@solana/wallet-adapter-react-ui';

export default function Navbar() {
  const pathname = usePathname();
  const isDashboard = pathname === '/dashboard';

  return (
    <nav className="flex justify-between items-center px-6 lg:px-8 py-4 border-b border-neutral-900/80 bg-[#050505]/80 backdrop-blur-xl sticky top-0 z-50">
      <div className="flex items-center gap-8">
        <Link href="/" className="flex items-center gap-3 group">
          <div className="flex items-center justify-center w-8 h-8 rounded-md border border-neutral-800 bg-neutral-900/50 group-hover:border-neutral-700 transition-colors">
            <Activity className="text-neutral-400 w-4 h-4 group-hover:text-neutral-300 transition-colors" />
          </div>
          <h1 className="text-lg font-medium tracking-tight text-neutral-100 group-hover:text-white transition-colors">
            SolBalance
          </h1>
        </Link>
        <div className="hidden md:flex gap-6 text-sm font-medium">
          <Link href="/" className={`transition-colors ${pathname === '/' ? 'text-white' : 'text-neutral-500 hover:text-neutral-300'}`}>Home</Link>
          <Link href="/dashboard" className={`transition-colors ${pathname === '/dashboard' ? 'text-white' : 'text-neutral-500 hover:text-neutral-300'}`}>App</Link>
          <Link href="/about" className={`transition-colors ${pathname === '/about' ? 'text-white' : 'text-neutral-500 hover:text-neutral-300'}`}>About</Link>
        </div>
      </div>
      
      {isDashboard ? (
        <WalletMultiButton className="!bg-[#0a0a0a] hover:!bg-neutral-900 !text-sm !font-medium !text-neutral-300 !h-9 !px-4 !rounded-md border border-neutral-800 !transition-all !shadow-none whitespace-nowrap" />
      ) : (
        <Link href="/dashboard" className="bg-neutral-100 hover:bg-white text-neutral-950 font-medium text-sm px-5 py-2 rounded-md transition-colors shadow-sm whitespace-nowrap flex items-center gap-2 relative group overflow-hidden">
          <span className="relative z-10">Launch App</span>
          <span className="relative z-10 transition-transform group-hover:translate-x-1">&rarr;</span>
        </Link>
      )}
    </nav>
  );
}
