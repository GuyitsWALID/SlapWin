import Navbar from "@/components/Navbar";
import Hero from "@/components/Hero";
import StatsBar from "@/components/StatsBar";
import Features from "@/components/Features";
import HowItWorks from "@/components/HowItWorks";
import VoicePacks from "@/components/VoicePacks";
import Trust from "@/components/Trust";
import FAQ from "@/components/FAQ";
import CTA from "@/components/CTA";
import Footer from "@/components/Footer";

export default function Home() {
  return (
    <main className="min-h-screen">
      <Navbar />
      <Hero />
      <StatsBar />
      <Features />
      <HowItWorks />
      <VoicePacks />
      <Trust />
      <FAQ />
      <CTA />
      <Footer />
    </main>
  );
}
