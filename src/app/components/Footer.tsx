import Link from "next/link";

export default function Footer() {
  return (
    <footer className="py-12 px-6 bg-dark border-t border-white/5">
      <div className="max-w-5xl mx-auto flex items-center justify-between flex-wrap gap-4">
        <p className="text-white/70 text-sm">© 2026 SlapWin. Built with Rust + Tauri. Made with 🖐️.</p>
        <div className="flex gap-6">
          <Link href="https://github.com/GuyitsWALID/SlapWin" target="_blank" className="text-white/70 hover:text-lime transition-colors text-sm">GitHub</Link>
          <Link href="#" className="text-white/70 hover:text-lime transition-colors text-sm">Privacy</Link>
          <Link href="#" className="text-white/70 hover:text-lime transition-colors text-sm">MIT License</Link>
        </div>
      </div>
    </footer>
  );
}
