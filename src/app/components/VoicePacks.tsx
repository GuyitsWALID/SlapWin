import { motion } from "framer-motion";

const packs = [
  { emoji: "😱", name: "Classic Scream", desc: "The iconic one from SlapMac", free: true },
  { emoji: "👻", name: "Horror", desc: "Scare anyone in the room", price: "$1.99" },
  { emoji: "🍕", name: "Comedy Club", desc: "Laughs, boos, and WOW", price: "$2.99" },
  { emoji: "👾", name: "Retro Game", desc: "8-bit sound effects", price: "$1.99" },
];

export default function VoicePacks() {
  return (
    <section id="packs" className="py-24 px-6">
      <div className="max-w-5xl mx-auto">
        <motion.div className="text-center mb-16" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }}>
          <h2 className="text-4xl font-bold mb-3 text-cream">Voice Packs 🎤</h2>
          <p className="text-white/70 text-lg">More than just screams. Choose your reaction.</p>
        </motion.div>
        <div className="grid grid-cols-2 lg:grid-cols-4 gap-4">
          {packs.map((p, i) => (
            <motion.div key={i} className="bg-dark border border-white/5 rounded-2xl p-6 text-center hover:border-coral hover:-translate-y-1 transition-all" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }} transition={{ delay: i * 0.08 }}>
              <div className="text-5xl mb-3">{p.emoji}</div>
              <h3 className="font-bold text-cream mb-1">{p.name}</h3>
              <p className="text-white/60 text-sm mb-3">{p.desc}</p>
              {p.free
                ? <span className="inline-block bg-lime/15 text-lime text-xs font-bold px-2 py-1 rounded">FREE</span>
                : <span className="inline-block bg-coral/15 text-coral text-xs font-bold px-2 py-1 rounded">{p.price}</span>}
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
