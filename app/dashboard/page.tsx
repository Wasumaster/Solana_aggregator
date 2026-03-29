'use client';

import React, { useState, useEffect } from 'react';
import DepositCard from '../components/DepositCard';
import { LineChart, Line, XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Area, AreaChart } from 'recharts';
import { ArrowUpRight, ArrowDownRight, Clock, Activity, Zap, ActivitySquare } from 'lucide-react';


const data1W = [
  { time: 'Mon', balance: 1000 }, { time: 'Tue', balance: 1002 }, { time: 'Wed', balance: 1005 },
  { time: 'Thu', balance: 1004 }, { time: 'Fri', balance: 1008 }, { time: 'Sat', balance: 1011 }, { time: 'Sun', balance: 1015 }
];
const data1M = [
  { time: 'W1', balance: 1000 }, { time: 'W2', balance: 1015 }, { time: 'W3', balance: 1040 }, { time: 'W4', balance: 1065 }
];
const data1Y = [
  { time: 'M1', balance: 1000 }, { time: 'M2', balance: 1065 }, { time: 'M4', balance: 1140 },
  { time: 'M6', balance: 1230 }, { time: 'M8', balance: 1350 }, { time: 'M10', balance: 1480 }, { time: 'M12', balance: 1650 }
];

const FUNDING_RATES = [
  { dex: 'Drift', market: 'SOL-PERP', rate: '0.0142%', apy: '48.5%' },
  { dex: 'Zeta', market: 'SOL-PERP', rate: '0.0094%', apy: '32.1%' },
  { dex: 'Jupiter', market: 'SOL-PERP', rate: '0.0112%', apy: '38.4%' },
  { dex: 'Flash', market: 'SOL-PERP', rate: '0.0081%', apy: '27.6%' },
];

const ACTIVITY_LOG = [
  { id: 1, type: 'automated_rebalance', title: 'Automated Rebalance', desc: 'Drift short adjusted to maintain delta 0.', time: '2m ago', tx: 'G5xT...9YqX' },
  { id: 2, type: 'harvest_yield', title: 'Harvested JitoSOL Yield', desc: '+ 0.041 JitoSOL added to pool.', time: '4h ago', tx: '3xK9...M1Po' },
  { id: 3, type: 'automated_rebalance', title: 'Automated Rebalance', desc: 'Zeta short adjusted to maintain delta 0.', time: '12h ago', tx: 'H9aC...8Pxz' },
  { id: 4, type: 'user_deposit', title: 'User Deposit', desc: '+ 1,000.00 USDC received in smart contract.', time: '1d ago', tx: '9mNs...Q1vR' },
];

export default function Dashboard() {
  const [solPrice, setSolPrice] = useState<number | null>(null);
  const [isLoadingPrice, setIsLoadingPrice] = useState(true);
  const [chartRange, setChartRange] = useState<'1W' | '1M' | '1Y'>('1Y');

  const activeData = chartRange === '1W' ? data1W : chartRange === '1M' ? data1M : data1Y;

  useEffect(() => {
    const fetchPrice = async () => {
      try {
        const res = await fetch('https://api.coingecko.com/api/v3/simple/price?ids=solana&vs_currencies=usd');
        const data = await res.json();
        if (data?.solana?.usd) setSolPrice(data.solana.usd);
      } catch (err) {
        console.error("Failed to fetch SOL price", err);
      } finally {
        setIsLoadingPrice(false);
      }
    };
    fetchPrice();
    const interval = setInterval(fetchPrice, 30000);
    return () => clearInterval(interval);
  }, []);

  return (
    <div className="container mx-auto max-w-7xl px-4 lg:px-8 py-10 space-y-6">
      
      {/* --- ROW 1: 4 KPIs --- */}
      <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
        <div className="p-5 border border-neutral-800/60 bg-[#0a0a0a] rounded-xl flex flex-col justify-center">
          <span className="text-xs font-medium text-neutral-500 tracking-wide uppercase mb-1">Total Balance</span>
          <span className="text-3xl font-medium tracking-tight text-neutral-100">$0.00</span>
        </div>
        
        <div className="p-5 border border-neutral-800/60 bg-[#0a0a0a] rounded-xl flex flex-col justify-center">
          <span className="text-xs font-medium text-neutral-500 tracking-wide uppercase mb-1 flex items-center gap-2">
            All-time Profit <ArrowUpRight className="w-3 h-3 text-emerald-500" />
          </span>
          <span className="text-3xl font-medium tracking-tight text-emerald-400">+$0.00</span>
        </div>

        <div className="p-5 border border-neutral-800/60 bg-[#0a0a0a] rounded-xl flex flex-col justify-center">
          <span className="text-xs font-medium text-neutral-500 tracking-wide uppercase mb-1">Live Net APY</span>
          <span className="text-3xl font-medium tracking-tight text-neutral-100">24.58%</span>
        </div>

        <div className="p-5 border border-neutral-800/60 bg-[#0a0a0a] rounded-xl flex flex-col justify-center relative overflow-hidden group">
          <span className="text-xs font-medium text-neutral-500 tracking-wide uppercase mb-1 relative z-10 flex items-center gap-2">
            SOL Price (Live)
            {isLoadingPrice ? <Activity className="w-3 h-3 animate-pulse" /> : <div className="w-1.5 h-1.5 rounded-full bg-emerald-500 animate-pulse"></div>}
          </span>
          <div className="flex items-baseline gap-2 relative z-10">
            {isLoadingPrice ? (
              <span className="text-3xl font-medium tracking-tight text-neutral-300">---</span>
            ) : (
              <>
                <span className="text-3xl font-medium tracking-tight text-neutral-100">${solPrice?.toFixed(2) || '184.20'}</span>
                <span className="text-xs text-neutral-500">USD</span>
              </>
            )}
          </div>
        </div>
      </div>

      {/* --- ROW 2: CHART & SIDEBAR ACTIONS --- */}
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        
        {/* Main Interactive Chart (Takes up 2/3) */}
        <div className="lg:col-span-2 border border-neutral-800/60 bg-[#0a0a0a] p-5 rounded-xl flex flex-col min-h-[400px]">
          <div className="flex flex-col sm:flex-row justify-between items-start sm:items-center mb-6 gap-4">
            <h3 className="text-sm font-medium text-neutral-100 uppercase tracking-wider">
              Yield Projection & Performance
            </h3>
            
            {/* Time Controls */}
            <div className="flex bg-neutral-900 border border-neutral-800 rounded-md p-1">
              {(['1W', '1M', '1Y'] as const).map(range => (
                <button 
                  key={range}
                  onClick={() => setChartRange(range)}
                  className={`px-3 py-1 text-xs font-medium rounded transition-colors ${chartRange === range ? 'bg-neutral-800 text-neutral-100' : 'text-neutral-500 hover:text-neutral-300'}`}
                >
                  {range}
                </button>
              ))}
            </div>
          </div>
          
          <div className="flex-1 w-full relative">
            <ResponsiveContainer width="100%" height="100%">
              <AreaChart data={activeData} margin={{ top: 10, right: 10, left: -20, bottom: 0 }}>
                <defs>
                  <linearGradient id="colorBalance" x1="0" y1="0" x2="0" y2="1">
                    <stop offset="5%" stopColor="#10b981" stopOpacity={0.2}/>
                    <stop offset="95%" stopColor="#10b981" stopOpacity={0}/>
                  </linearGradient>
                </defs>
                <CartesianGrid strokeDasharray="3 3" stroke="#171717" vertical={false} />
                <XAxis dataKey="time" stroke="#404040" fontSize={11} tickLine={false} axisLine={false} dy={10} />
                <YAxis stroke="#404040" fontSize={11} tickLine={false} axisLine={false} tickFormatter={(value) => `$${value}`} />
                <Tooltip 
                  contentStyle={{ backgroundColor: '#0a0a0a', borderColor: '#262626', borderRadius: '8px', color: '#f5f5f5', fontSize: '13px' }}
                  itemStyle={{ color: '#10b981' }}
                  formatter={(value) => [`$${value}`, 'Proj. Balance']}
                />
                <Area type="monotone" dataKey="balance" stroke="#10b981" strokeWidth={2} fillOpacity={1} fill="url(#colorBalance)" />
              </AreaChart>
            </ResponsiveContainer>
          </div>
        </div>

        {/* Sidebar (Takes up 1/3) */}
        <div className="flex flex-col gap-6">
          <DepositCard />

          {/* Portfolio Breakdown */}
          <div className="border border-neutral-800/60 bg-[#0a0a0a] rounded-xl p-5">
            <h3 className="text-sm font-medium text-neutral-100 mb-5 uppercase tracking-wider">
              Portfolio Breakdown
            </h3>
            
            <div className="mb-4">
              <div className="flex justify-between items-center mb-2">
                <span className="text-xs text-neutral-400 font-medium">JitoSOL (Spot Long)</span>
                <span className="text-xs text-neutral-100 font-mono">50.0%</span>
              </div>
              <div className="w-full h-1.5 bg-neutral-900 rounded-full overflow-hidden">
                <div className="h-full bg-emerald-500 w-[50%]"></div>
              </div>
            </div>

            <div>
              <div className="flex justify-between items-center mb-2">
                <span className="text-xs text-neutral-400 font-medium flex items-center gap-1">
                  Drift PERP (Short) <ArrowDownRight className="w-3 h-3 text-red-500" />
                </span>
                <span className="text-xs text-neutral-100 font-mono">50.0%</span>
              </div>
              <div className="w-full h-1.5 bg-neutral-900 rounded-full overflow-hidden">
                <div className="h-full bg-neutral-500 w-[50%]"></div>
              </div>
            </div>

            <div className="mt-5 pt-4 border-t border-neutral-800/50 flex justify-between items-center">
              <span className="text-[11px] text-neutral-500 uppercase tracking-widest">Delta Exposure</span>
              <span className="text-xs font-bold text-neutral-100 bg-neutral-900 px-2 py-1 rounded border border-neutral-800">Perfect 0</span>
            </div>
          </div>
        </div>
      </div>

      {/* --- ROW 3: TABLES --- */}
      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        
        {/* Live Funding Rates Table */}
        <div className="border border-neutral-800/60 bg-[#0a0a0a] rounded-xl overflow-hidden flex flex-col">
          <div className="p-5 border-b border-neutral-800/60 bg-neutral-900/20">
            <h3 className="text-sm font-medium text-neutral-100 uppercase tracking-wider">
              1h Live Funding Rates
            </h3>
          </div>
          <div className="p-0 overflow-x-auto">
            <table className="w-full text-left text-sm whitespace-nowrap">
              <thead className="bg-neutral-900/10 text-neutral-500 text-xs uppercase tracking-wider">
                <tr>
                  <th className="px-5 py-3 font-medium">Platform</th>
                  <th className="px-5 py-3 font-medium">Market</th>
                  <th className="px-5 py-3 font-medium text-right">Avg Rate (1h)</th>
                  <th className="px-5 py-3 font-medium text-right">Proj. APY</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-neutral-800/50">
                {FUNDING_RATES.map((row, i) => (
                  <tr key={i} className="hover:bg-neutral-900/30 transition-colors">
                    <td className="px-5 py-4 text-neutral-300 font-medium">{row.dex}</td>
                    <td className="px-5 py-4 text-neutral-500">{row.market}</td>
                    <td className="px-5 py-4 text-emerald-400 text-right font-mono">{row.rate}</td>
                    <td className="px-5 py-4 text-neutral-100 text-right font-mono">{row.apy}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          </div>
        </div>

        {/* Automated Activity Log */}
        <div className="border border-neutral-800/60 bg-[#0a0a0a] rounded-xl overflow-hidden flex flex-col">
          <div className="p-5 border-b border-neutral-800/60 bg-neutral-900/20 flex justify-between items-center">
            <h3 className="text-sm font-medium text-neutral-100 uppercase tracking-wider">
              Algorithm Activity Log
            </h3>
            <span className="flex items-center gap-2 text-xs text-neutral-500">
              <ActivitySquare className="w-3.5 h-3.5" /> Engine Active
            </span>
          </div>
          <div className="p-0 overflow-y-auto max-h-[300px]">
             {ACTIVITY_LOG.map((log) => (
                <div key={log.id} className="p-5 border-b border-neutral-800/50 hover:bg-neutral-900/20 transition-colors flex gap-4 items-start">
                  <div className={`mt-0.5 w-6 h-6 rounded-md flex items-center justify-center border ${
                    log.type === 'automated_rebalance' ? 'border-neutral-700 text-neutral-400 bg-neutral-800/50' :
                    log.type === 'harvest_yield' ? 'border-emerald-500/30 text-emerald-400 bg-emerald-500/10' :
                    'border-blue-500/30 text-blue-400 bg-blue-500/10'
                  }`}>
                    {log.type === 'automated_rebalance' ? <Clock className="w-3.5 h-3.5" /> : 
                     log.type === 'harvest_yield' ? <Zap className="w-3.5 h-3.5" /> : 
                     <ActivitySquare className="w-3.5 h-3.5" />}
                  </div>
                  <div className="flex-1">
                    <div className="flex justify-between items-start mb-1">
                      <span className="text-sm font-medium text-neutral-200">{log.title}</span>
                      <span className="text-xs text-neutral-500 font-mono">{log.time}</span>
                    </div>
                    <p className="text-xs text-neutral-400 leading-relaxed mb-2">{log.desc}</p>
                    <div className="text-[10px] text-neutral-600 font-mono border border-neutral-800 inline-block px-1.5 py-0.5 rounded">
                      TX: {log.tx}
                    </div>
                  </div>
                </div>
             ))}
          </div>
        </div>
        
      </div>

    </div>
  );
}
