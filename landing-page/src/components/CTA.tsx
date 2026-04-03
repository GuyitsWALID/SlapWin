import { motion } from "framer-motion";
import Link from "next/link";

export default function CTA() {
  return (
    <section id="download" className="py-24 px-6 text-center relative overflow-hidden">
      <div className="absolute inset-0 bg-[radial-gradient(50%_50%_at_30%_50%,rgba(164,230,45,0.06),transparent),radial-gradient(50%_50%_at_70%_50%,rgba(255,107,107,0.04),transparent)]" />
      <div className="relative max-w-3xl mx-auto z-10">
        <motion.div initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }}>
          <motion.h1 className="font-hand text-6xl font-bold text-coral mb-4" style={{ textShadow: "4px 4px 0 #A4E62D, -2px -2px 0 #FFF8E7" }} animate={{ rotate: [-3, 3, -3] }} transition={{ repeat: Infinity, duration: 3, ease: "ease-in-out" }}>SLAP!</motion.h1>
          <h2 className="text-5xl font-extrabold text-cream mb-4">Ready to get <span className="font-hand text-coral">slapping</span>?</h2>
          <p className="text-white/70 text-lg mb-8">Download for free. No sign-up. No email. Just slap.</p>
          <div className="flex items-center justify-center gap-4 flex-wrap mb-4">
            <Link href="#" className="inline-flex items-center gap-3 bg-gradient-to-br from-lime to-lime-dark text-dark font-bold text-lg px-8 py-4 rounded-xl shadow-lg shadow-lime/25 hover:-translate-y-1 hover:scale-105 transition-all">
              <svg width="22" height="22" fill="none" stroke="currentColor" strokeWidth="2.5" strokeLinecap="round" strokeLinejoin="round" viewBox="0 0 24 24"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline points="7 10 12 15 17 10" /><line x1="12" y1="15" x2="12" y2="3" /></svg>
              Download .exe
            </Link>
            <Link href="https://github.com/GuyitsWALID/SlapWin" target="_blank" className="inline-flex items-center gap-2 bg-transparent text-cream border-2 border-white/20 px-6 py-3 rounded-xl font-semibold hover:border-lime hover:text-lime transition-all">
              <svg width="20" height="20" fill="currentColor" viewBox="0 0 24 24"><path d="M12 0C5.374 0 0 5.373 0 12c0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.066 1.819 1.237 1.819 1.237 1.072 1.841 2.812 1.307 3.497.996.107-.777.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.47-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23A11.509 11.509 0 0 1 12 5.803c1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576C20.566 21.797 24 17.3 24 12c0-6.627-5.373-12-12-12z" /></svg>
              View on GitHub
            </Link>
          </div>
          <p className="text-white/40 text-sm">Windows 10/11 • 64-bit • Free • Under 10MB • No install required</p>
        </motion.div>
      </div>
    </section>
  );
}
