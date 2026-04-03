"use client";
import { motion } from "framer-motion";

const stats = [
  { num: "<10MB", label: "Binary Size" },
  { num: "<100MB", label: "RAM Usage" },
  { num: "<200ms", label: "Cold Start" },
  { num: "Zero", label: "Dependencies" },
];

export default function StatsBar() {
  return (
    <section className="py-16 bg-dark-light border-t border-white/5">
      <div className="max-w-4xl mx-auto px-6 grid grid-cols-2 md:grid-cols-4 gap-8 text-center">
        {stats.map((stat, i) => (
          <motion.div
            key={i}
            initial={{ opacity: 0, y: 20 }}
            whileInView={{ opacity: 1, y: 0 }}
            viewport={{ once: true }}
            transition={{ delay: i * 0.1 }}
          >
            <div className="text-4xl md:text-5xl font-extrabold text-lime mb-1">{stat.num}</div>
            <div className="text-white/70 text-sm">{stat.label}</div>
          </motion.div>
        ))}
      </div>
    </section>
  );
}
