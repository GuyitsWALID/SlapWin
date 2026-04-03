"use client";
import { motion } from "framer-motion";

const items = [
  { icon: "🔒", title: "No data collection", desc: "Zero telemetry. No accounts. Your slaps stay private." },
  { icon: "⚡", title: "Ultra lightweight", desc: "Under 10MB. Uses less RAM than a browser tab." },
  { icon: "🛡️", title: "Safe code", desc: "Built with Rust — no memory vulnerabilities." },
  { icon: "💰", title: "Free forever", desc: "Core app is always free. Pay once for voice packs." },
];

export default function Trust() {
  return (
    <section className="py-20 px-6 bg-dark-light border-t border-white/5">
      <div className="max-w-4xl mx-auto grid md:grid-cols-2 gap-6">
        {items.map((item, i) => (
          <motion.div key={i} className="flex items-start gap-4" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }} transition={{ delay: i * 0.1 }}>
            <div className="w-10 h-10 bg-lime/10 rounded-lg flex items-center justify-center text-xl shrink-0">{item.icon}</div>
            <div>
              <h4 className="font-semibold text-cream mb-1">{item.title}</h4>
              <p className="text-white/60 text-sm">{item.desc}</p>
            </div>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
