import { motion } from "framer-motion";

const steps = [
  { num: "1", title: "Download & Run", desc: "Grab the ~8MB .exe. Run it, give mic permission. That's it. No admin install needed." },
  { num: "2", title: "Calibrate", desc: "Open the sensitivity dial. Test it — tap, slap, or nudge. Set the trigger to your vibe." },
  { num: "3", title: "SLAP! 🎉", desc: "Go full slap mode. Share on TikTok. Watch it go viral. You're welcome." },
];

export default function HowItWorks() {
  return (
    <section id="how" className="py-24 px-6 bg-cream text-dark">
      <div className="max-w-4xl mx-auto">
        <motion.div className="text-center mb-16" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }}>
          <h2 className="text-4xl font-bold mb-3">How it <span className="text-coral-dark">works</span></h2>
          <p className="text-gray-600 text-lg">From download to slap-induced comedy in under a minute.</p>
        </motion.div>
        <div className="grid md:grid-cols-3 gap-8">
          {steps.map((s, i) => (
            <motion.div key={i} className="text-center" initial={{ opacity: 0, y: 20 }} whileInView={{ opacity: 1, y: 0 }} viewport={{ once: true }} transition={{ delay: i * 0.15 }}>
              <div className="w-14 h-14 bg-dark text-lime rounded-2xl font-hand text-2xl flex items-center justify-center mx-auto mb-4">{s.num}</div>
              <h3 className="text-xl font-bold mb-2">{s.title}</h3>
              <p className="text-gray-600">{s.desc}</p>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
}
