"use client";
import { useState } from "react";
import { motion } from "framer-motion";

const faqs = [
  { q: "Why does it need microphone access?", a: "It listens for sudden amplitude spikes (the slap/clap). It does NOT record or stream audio. Processing is 100% local." },
  { q: "Is this a prank app?", a: "We call it comedic software engineering. SlapMac hit 10M+ views proving desktop comedy is a real category." },
  { q: "Will it work on my Windows laptop?", a: "Any Windows 10/11 laptop with a built-in mic works. Desktops with external mics work too. Sensitivity is adjustable." },
  { q: "Can I use my own sounds?", a: "Absolutely! Import any .wav or .mp3 file as a reaction. Create custom voice packs to share with friends." },
  { q: "Will it slow down my PC?", a: "Nope. Sits quietly in the system tray using under 100MB RAM. No background processes, lighter than a cat video tab." },
];

export default function FAQ() {
  const [open, setOpen] = useState<number | null>(null);
  return (
    <section id="faq" className="py-24 px-6">
      <div className="max-w-3xl mx-auto">
        <motion.div className="text-center mb-16" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }}>
          <h2 className="text-4xl font-bold mb-3 text-cream">Got <span className="text-coral">questions</span>?</h2>
          <p className="text-white/70 text-lg">Everything you need to know before your first slap.</p>
        </motion.div>
        <div className="space-y-0">
          {faqs.map((faq, i) => (
            <div key={i} className="border-b border-white/5">
              <button className="w-full flex items-center justify-between gap-4 text-left text-cream font-semibold py-5 hover:text-lime transition-colors" onClick={() => setOpen(open === i ? null : i)}>
                <span>{faq.q}</span>
                <span className="text-2xl text-lime shrink-0" style={{ transform: open === i ? "rotate(45deg)" : "none", transition: "transform 0.3s" }}>+</span>
              </button>
              <div className="overflow-hidden transition-all duration-300" style={{ maxHeight: open === i ? "200px" : "0" }}>
                <p className="text-white/60 text-sm leading-relaxed pb-4">{faq.a}</p>
              </div>
            </div>
          ))}
        </div>
      </div>
    </section>
  );
}
