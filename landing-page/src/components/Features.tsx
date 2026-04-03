import { motion } from "framer-motion";

const features = [
  { icon: "🖐️", title: "Mic Detection", desc: "Uses your built-in mic to detect sharp amplitude spikes — clap, desk slam, or gentle tap." },
  { icon: "⌨️", title: "Keyboard Patterns", desc: "Timing-based detection when multiple keys are pressed rapidly — like when you slam your hands down." },
  { icon: "🔊", title: "Instant Audio", desc: "Low-latency sound via Windows Core Audio APIs. Reactions fire within milliseconds of impact." },
  { icon: "🎭", title: "Voice Packs", desc: "Start with the classic scream. Unlock comedy, horror, anime, or retro game packs." },
  { icon: "🎛️", title: "Sensitivity Dial", desc: "Built-in tuner sets the exact trigger threshold for noisy cafes and quiet bedrooms alike." },
  { icon: "🔒", title: "Privacy First", desc: "Zero data collection. Audio never leaves your machine. No accounts, no cloud, local processing only." },
];

export default function Features() {
  return (
    <section id="features" className="py-24 px-6">
      <div className="max-w-5xl mx-auto">
        <motion.div className="text-center mb-16" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }}>
          <h2 className="text-4xl font-bold mb-3 text-cream">What is <span className="text-coral">SlapWin</span>? 🤔</h2>
          <p className="text-white/70 text-lg max-w-2xl mx-auto">A utility that plays funny audio reactions when you slap your laptop lid, slam your desk, or trigger USB events.</p>
        </motion.div>
        <div className="grid md:grid-cols-2 lg:grid-cols-3 gap-6">
          {features.map((f, i) => (
            <motion.div key={i} className="bg-dark-light border border-white/5 rounded-2xl p-8 hover:border-lime/30 hover:-translate-y-1 transition-all" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }} transition={{ delay: i * 0.08 }}>
              <div className="w-14 h-14 bg-lime/10 rounded-xl flex items-center justify-center text-2xl mb-5">{f.icon}</div>
              <h3 className="text-lg font-semibold text-cream mb-2">{f.title}</h3>
              <p className="text-white/60 text-sm leading-relaxed">{f.desc}</p>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
