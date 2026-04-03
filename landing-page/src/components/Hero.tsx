import { motion } from "framer-motion";
import Link from "next/link";

export default function Hero() {
  return (
    <section className="min-h-screen flex items-center justify-center relative overflow-hidden pt-24 pb-16">
      <div className="absolute -top-[30%] -left-[20%] w-[80%] h-[150%] bg-[radial-gradient(ellipse,rgba(164,230,45,0.08)_0%,transparent_70%)] animate-[pulse_8s_ease-in-out_infinite]" />

      <div className="max-w-4xl mx-auto px-6 text-center relative z-10">
        <motion.div
          className="w-[180px] h-[180px] mx-auto mb-8"
          animate={{ rotate: [-5, -3, 5, -3, -5], scale: [1, 1.05, 0.95, 1.02, 1] }}
          transition={{ repeat: Infinity, duration: 2, ease: "ease-in-out" }}
        >
          <svg viewBox="0 0 200 200" xmlns="http://www.w3.org/2000/svg">
            <rect x="40" y="30" width="120" height="90" rx="8" ry="8" fill="#FFF8E7" stroke="#1A1A2E" strokeWidth="3" />
            <rect x="50" y="40" width="100" height="65" rx="4" ry="4" fill="#FF6B6B" stroke="#1A1A2E" strokeWidth="2.5" />
            <circle cx="75" cy="60" r="10" fill="#FFF8E7" stroke="#1A1A2E" strokeWidth="2" />
            <ellipse cx="77" cy="61" rx="4" ry="6" fill="#1A1A2E" />
            <circle cx="125" cy="60" r="10" fill="#FFF8E7" stroke="#1A1A2E" strokeWidth="2" />
            <ellipse cx="127" cy="61" rx="4" ry="6" fill="#1A1A2E" />
            <path d="M64 48 Q70 42 85 46" stroke="#1A1A2E" strokeWidth="2.5" fill="none" />
            <path d="M115 46 Q130 42 136 48" stroke="#1A1A2E" strokeWidth="2.5" fill="none" />
            <ellipse cx="100" cy="84" rx="16" ry="13" fill="#1A1A2E" />
            <path d="M40 60 Q20 45 10 55 Q15 70 35 75" fill="none" stroke="#1A1A2E" strokeWidth="3" strokeLinecap="round" />
            <path d="M160 60 Q180 45 190 55 Q185 70 165 75" fill="none" stroke="#1A1A2E" strokeWidth="3" strokeLinecap="round" />
            <line x1="90" y1="120" x2="75" y2="165" stroke="#1A1A2E" strokeWidth="3" strokeLinecap="round" />
            <line x1="110" y1="120" x2="125" y2="165" stroke="#1A1A2E" strokeWidth="3" strokeLinecap="round" />
            <path d="M150 80 L155 90 L165 92 L158 100 L158 112 L148 106 L138 112 L138 100 L130 92 L140 90" fill="#A4E62D" stroke="#1A1A2E" strokeWidth="1.5" />
          </svg>
        </motion.div>

        <motion.h1
          className="font-hand text-7xl font-bold text-coral mb-4"
          style={{ textShadow: "4px 4px 0 #A4E62D, -2px -2px 0 #FFF8E7" }}
          animate={{ rotate: [-3, 3, -3] }}
          transition={{ repeat: Infinity, duration: 3, ease: "ease-in-out" }}
        >
          SLAP!
        </motion.h1>

        <div className="inline-flex items-center gap-2 bg-coral/12 border border-coral text-coral px-4 py-2 rounded-full text-sm font-semibold mb-6">
          🔥 SlapMac went viral. Now Windows gets one too.
        </div>

        <h2 className="text-5xl font-extrabold text-cream mb-4">
          The funniest <span className="text-lime">utility</span> you will ever slap.
        </h2>

        <p className="text-xl text-white/70 mb-10 max-w-xl mx-auto leading-relaxed">
          Slap your laptop and get a <strong className="text-lime">comedic scream</strong>. Built with Tauri, under <strong className="text-lime">10MB</strong>, zero install. Because why not?
        </p>

        <Link href="#download" className="inline-flex items-center gap-3 bg-gradient-to-br from-lime to-lime-dark text-dark font-bold text-xl px-10 py-4 rounded-xl shadow-lg shadow-lime/25 hover:-translate-y-1 hover:scale-105 hover:shadow-xl hover:shadow-lime/40 transition-all mb-6">
          <svg width="24" height="24" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" viewBox="0 0 24 24">
            <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
            <polyline points="7 10 12 15 17 10" />
            <line x1="12" y1="15" x2="12" y2="3" />
          </svg>
          Download SlapWin — Free
        </Link>

        <div className="flex items-center justify-center gap-6 flex-wrap text-white/70 text-sm">
          <span>⚡ <span className="text-lime font-semibold">&lt;10MB</span></span>
          <span>🎯 <span className="text-lime font-semibold">&lt;100MB</span> RAM</span>
          <span>🖥️ Windows 10/11</span>
          <span>🌿 MIT License</span>
        </div>
      </div>
    </section>
  );
}
