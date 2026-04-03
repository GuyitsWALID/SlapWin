import Link from "next/link";

export default function Navbar() {
  return (
    <nav className="fixed top-0 w-full z-50 bg-dark/95 backdrop-blur-xl border-b border-lime/15">
      <div className="max-w-6xl mx-auto px-6 py-4 flex items-center justify-between">
        <Link href="#" className="font-hand text-3xl font-bold text-lime">
          Slap<span className="text-coral">Win</span>
        </Link>
        <ul className="hidden md:flex items-center gap-8">
          <li><Link href="#features" className="text-white/70 hover:text-lime transition-colors font-medium">Features</Link></li>
          <li><Link href="#how" className="text-white/70 hover:text-lime transition-colors font-medium">How It Works</Link></li>
          <li><Link href="#packs" className="text-white/70 hover:text-lime transition-colors font-medium">Voice Packs</Link></li>
          <li><Link href="#faq" className="text-white/70 hover:text-lime transition-colors font-medium">FAQ</Link></li>
          <li><Link href="#download" className="bg-lime text-dark font-bold px-5 py-2.5 rounded-lg hover:bg-lime-dark transition-colors hover:-translate-y-0.5 transform">Download Free</Link></li>
        </ul>
      </div>
    </nav>
  );
}
