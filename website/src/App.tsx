import { useState } from "react";
import { translations } from "./translations";
import { motion } from "motion/react";
import {
  Sliders,
  Layers,
  Volume2,
  ShieldCheck,
  Lock,
  Github,
  HeartHandshake,
  ChevronRight,
  Menu,
  X,
  VolumeX,
  Radio,
  FileCheck2,
  Cpu,
  RefreshCw,
  Sparkles,
  Laptop,
  Smartphone,
} from "lucide-react";

export default function App() {
  const [mobileMenuOpen, setMobileMenuOpen] = useState(false);
  const t = translations.no;

  const fadeInUp = {
    hidden: { opacity: 0, y: 30 },
    visible: {
      opacity: 1,
      y: 0,
      transition: { duration: 0.6, ease: "easeOut" as const },
    },
  };

  const staggerContainer = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: {
        staggerChildren: 0.15,
      },
    },
  };

  return (
    <div className="min-h-screen bg-surface text-text-primary selection:bg-primary/30 selection:text-text-primary">
      {/* BACKGROUND ATMOSPHERIC GLOWS */}
      <div className="absolute top-0 right-0 w-[500px] h-[500px] bg-primary/5 rounded-full blur-[120px] pointer-events-none"></div>
      <div className="absolute top-[40%] left-0 w-[600px] h-[600px] bg-status-destructive/5 rounded-full blur-[140px] pointer-events-none"></div>
      <div className="absolute bottom-0 right-0 w-[500px] h-[500px] bg-primary/5 rounded-full blur-[120px] pointer-events-none"></div>

      {/* TOP DOCK NAVIGATION */}
      <nav className="sticky top-0 z-50 bg-surface/90 backdrop-blur-md border-b border-outline-variant/20 transition-all duration-300">
        <div className="max-w-[1500px] mx-auto px-6 md:px-12 py-4 flex justify-between items-center">
          {/* Brand Logo */}
          <a href="#" className="flex items-center gap-3 group">
            <div className="flex gap-1 items-end h-5">
              <span className="w-1 bg-primary h-3 rounded-full group-hover:h-5 transition-all duration-300"></span>
              <span className="w-1 bg-primary h-5 rounded-full"></span>
              <span className="w-1 bg-primary h-4 rounded-full group-hover:h-3 transition-all duration-300"></span>
              <span className="w-1 bg-primary h-2 rounded-full group-hover:h-4 transition-all duration-300"></span>
            </div>
            <span className="font-serif text-2xl font-bold tracking-tight text-text-primary">
              Lydkontroll
            </span>
          </a>

          {/* Desktop Navigation */}
          <div className="hidden md:flex items-center gap-8">
            <a
              href="#how-it-works"
              className="text-sm font-medium text-text-secondary hover:text-text-primary transition-colors"
            >
              {t.navHow}
            </a>
            <a
              href="#story"
              className="text-sm font-medium text-text-secondary hover:text-text-primary transition-colors"
            >
              {t.navStory}
            </a>
            <a
              href="#robustness"
              className="text-sm font-medium text-text-secondary hover:text-text-primary transition-colors"
            >
              {t.navRobust}
            </a>
          </div>

          {/* Right Header Actions */}
          <div className="hidden md:flex items-center gap-4">
            {/* Main CTA */}
            <a
              href="https://github.com/dagjomar/lydkontroll"
              target="_blank"
              rel="noreferrer"
              className="px-5 py-2 bg-primary text-on-primary font-bold rounded-xl text-sm transition-all hover:scale-102 hover:shadow-lg active:scale-98"
            >
              Se kildekoden
            </a>
          </div>

          {/* Mobile Menu Button */}
          <div className="flex items-center gap-2 md:hidden">
            <button
              onClick={() => setMobileMenuOpen(!mobileMenuOpen)}
              className="p-2 rounded-lg bg-surface-container border border-outline-variant/10 text-text-primary hover:bg-surface-bright transition-colors"
            >
              {mobileMenuOpen ? <X size={20} /> : <Menu size={20} />}
            </button>
          </div>
        </div>

        {/* Mobile Navigation Dropdown */}
        {mobileMenuOpen && (
          <motion.div
            initial={{ opacity: 0, y: -10 }}
            animate={{ opacity: 1, y: 0 }}
            className="md:hidden border-t border-outline-variant/20 bg-surface px-6 py-6 space-y-4"
          >
            <a
              href="#how-it-works"
              onClick={() => setMobileMenuOpen(false)}
              className="block text-base font-semibold text-text-secondary hover:text-text-primary"
            >
              {t.navHow}
            </a>
            <a
              href="#story"
              onClick={() => setMobileMenuOpen(false)}
              className="block text-base font-semibold text-text-secondary hover:text-text-primary"
            >
              {t.navStory}
            </a>
            <a
              href="#robustness"
              onClick={() => setMobileMenuOpen(false)}
              className="block text-base font-semibold text-text-secondary hover:text-text-primary"
            >
              {t.navRobust}
            </a>
            <div className="pt-4 border-t border-outline-variant/10 flex flex-col gap-3">
              <a
                href="https://github.com/dagjomar/lydkontroll"
                target="_blank"
                rel="noreferrer"
                onClick={() => setMobileMenuOpen(false)}
                className="w-full bg-primary text-on-primary text-center py-3 rounded-xl font-bold text-sm"
              >
                Se kildekoden
              </a>
            </div>
          </motion.div>
        )}
      </nav>

      {/* HERO SECTION */}
      <section className="relative pt-16 pb-28 px-6 md:px-12 max-w-[1500px] mx-auto overflow-hidden">
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-16 items-center">
          {/* Hero Left Copy */}
          <motion.div
            initial="hidden"
            animate="visible"
            variants={fadeInUp}
            className="lg:col-span-6 text-center lg:text-left z-10"
          >
            <span className="font-sans text-xs font-extrabold tracking-widest text-primary uppercase bg-primary/10 border border-primary/20 px-3.5 py-1.5 rounded-full inline-block mb-6">
              {t.heroEyebrow}
            </span>
            <h1 className="font-serif text-5xl md:text-6xl xl:text-7xl font-bold text-text-primary leading-[1.05] tracking-tight mb-6">
              {t.heroTitleLine1} <br />
              <span className="italic text-primary font-normal">
                {t.heroTitleLine2}
              </span>
            </h1>
            <p className="text-text-secondary text-base md:text-lg leading-relaxed max-w-xl mx-auto lg:mx-0 mb-10">
              {t.heroDesc}
            </p>

            <div className="flex flex-col sm:flex-row gap-4 justify-center lg:justify-start">
              <a
                href="https://github.com/dagjomar/lydkontroll"
                target="_blank"
                rel="noreferrer"
                className="px-8 py-4 bg-primary text-on-primary font-bold rounded-xl flex items-center justify-center gap-3 transition-transform hover:scale-103 shadow-lg shadow-primary/5 active:scale-98"
              >
                <Github size={18} />
                <span>{t.heroCtaSource}</span>
              </a>
              <a
                href="#how-it-works"
                className="px-8 py-4 bg-surface-container hover:bg-surface-bright text-text-primary font-bold rounded-xl border border-outline-variant/30 flex items-center justify-center transition-all hover:border-outline-variant/50 active:scale-98"
              >
                <span>{t.heroCtaDemo}</span>
              </a>
            </div>
          </motion.div>

          {/* Hero Right Images Showcase */}
          <motion.div
            initial={{ opacity: 0, scale: 0.95, rotate: 1 }}
            animate={{ opacity: 1, scale: 1, rotate: -1.5 }}
            transition={{ duration: 0.8, ease: "easeOut", delay: 0.2 }}
            className="lg:col-span-6 relative flex justify-center items-center mt-12 lg:mt-0"
          >
            <div className="relative w-full max-w-2xl transform transition-transform duration-700 hover:rotate-0">
              {/* Backlight Glow */}
              <div className="absolute -inset-4 bg-gradient-to-tr from-primary/15 to-transparent rounded-3xl blur-2xl"></div>

              {/* Main Desktop Interface Hotlink */}
              <img
                src="./images/desktop-overview.png?v=2"
                alt="Lydkontroll Main Workstation"
                referrerPolicy="no-referrer"
                className="rounded-2xl shadow-2xl border border-white/10 w-full object-cover"
              />

              {/* Overlapping Mobile Device frame */}
              <motion.div
                initial={{ y: 20, rotate: 4 }}
                animate={{ y: 0, rotate: 3 }}
                transition={{
                  duration: 1,
                  delay: 0.4,
                  type: "spring",
                  stiffness: 60,
                }}
                className="absolute -bottom-10 -left-6 md:-left-12 w-48 md:w-64 shadow-2xl z-20 border-[6px] border-surface-dim rounded-[2.5rem] overflow-hidden"
              >
                <img
                  src="./images/mobile-control.png?v=2"
                  alt="Lydkontroll Mobile App Overlay"
                  referrerPolicy="no-referrer"
                  className="w-full object-cover"
                />
              </motion.div>
            </div>
          </motion.div>
        </div>
      </section>

      {/* HOW IT WORKS / SLIK VIRKER DET SECTION */}
      <section
        className="py-28 bg-surface-container-lowest border-y border-outline-variant/10"
        id="how-it-works"
      >
        <div className="max-w-[1500px] mx-auto px-6 md:px-12">
          {/* Header section info */}
          <div className="text-center mb-20">
            <span className="font-sans text-xs font-extrabold tracking-widest text-primary uppercase block mb-3">
              {t.workflowEyebrow}
            </span>
            <h2 className="font-serif text-4xl md:text-5xl text-text-primary font-medium tracking-tight mb-4">
              {t.workflowTitle}
            </h2>
            <p className="text-text-secondary text-base max-w-2xl mx-auto leading-relaxed">
              {t.workflowDesc}
            </p>
          </div>

          <div className="grid grid-cols-1 lg:grid-cols-2 gap-16 items-center">
            {/* Steps Left graphics */}
            <motion.div
              initial="hidden"
              whileInView="visible"
              viewport={{ once: true }}
              variants={staggerContainer}
              className="relative order-2 lg:order-1"
            >
              <div className="aspect-video bg-surface-container rounded-2xl border border-outline-variant/10 overflow-hidden relative group shadow-xl">
                <img
                  src="./images/mac-workflow.png?v=2"
                  alt="Lydkontroll System Architecture"
                  referrerPolicy="no-referrer"
                  className="absolute inset-0 w-full h-full object-cover opacity-50 group-hover:opacity-75 transition-opacity duration-500"
                />

                {/* Visual architectural link labels inside graphic */}
                <div className="absolute inset-0 z-10 flex items-center justify-between px-12 md:px-20 py-8 bg-gradient-to-t from-surface-container-lowest via-transparent to-transparent">
                  <div className="flex flex-col items-center gap-3">
                    <div className="w-16 h-16 rounded-xl bg-surface-container-highest border border-primary/40 flex items-center justify-center shadow-lg">
                      <Laptop size={28} className="text-primary" />
                    </div>
                    <span className="text-[10px] font-bold text-text-primary tracking-widest font-sans uppercase">
                      MAC (CORE)
                    </span>
                  </div>

                  <div className="flex-1 flex flex-col items-center gap-1.5 px-6">
                    <div className="h-0.5 w-full bg-gradient-to-r from-primary/10 via-primary to-primary/10 animate-pulse"></div>
                    <span className="text-[9px] text-primary font-bold tracking-widest font-sans uppercase">
                      TAILSCALE LINK
                    </span>
                  </div>

                  <div className="flex flex-col items-center gap-3">
                    <div className="w-16 h-16 rounded-xl bg-surface-container-highest border border-primary/40 flex items-center justify-center shadow-lg">
                      <Smartphone size={28} className="text-primary" />
                    </div>
                    <span className="text-[10px] font-bold text-text-primary tracking-widest font-sans uppercase">
                      IPHONE (REMOTE)
                    </span>
                  </div>
                </div>
              </div>
            </motion.div>

            {/* Steps Right Descriptions */}
            <div className="order-1 lg:order-2 space-y-10">
              <div className="flex gap-6 items-start">
                <div className="flex-shrink-0 w-10 h-10 rounded-xl bg-primary/10 border border-primary/30 text-primary flex items-center justify-center font-bold text-base font-serif">
                  1
                </div>
                <div>
                  <h3 className="font-serif text-2xl text-text-primary font-medium tracking-tight mb-2">
                    {t.step1Title}
                  </h3>
                  <p className="text-text-muted text-sm leading-relaxed">
                    {t.step1Desc}
                  </p>
                </div>
              </div>

              <div className="flex gap-6 items-start">
                <div className="flex-shrink-0 w-10 h-10 rounded-xl bg-primary/10 border border-primary/30 text-primary flex items-center justify-center font-bold text-base font-serif">
                  2
                </div>
                <div>
                  <h3 className="font-serif text-2xl text-text-primary font-medium tracking-tight mb-2">
                    {t.step2Title}
                  </h3>
                  <p className="text-text-muted text-sm leading-relaxed">
                    {t.step2Desc}
                  </p>
                </div>
              </div>

              <div className="flex gap-6 items-start">
                <div className="flex-shrink-0 w-10 h-10 rounded-xl bg-primary/10 border border-primary/30 text-primary flex items-center justify-center font-bold text-base font-serif">
                  3
                </div>
                <div>
                  <h3 className="font-serif text-2xl text-text-primary font-medium tracking-tight mb-2">
                    {t.step3Title}
                  </h3>
                  <p className="text-text-muted text-sm leading-relaxed">
                    {t.step3Desc}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* USABILITY SCREENSHOW / EXTRA PRODUCT VIEW SECTION */}
      <section className="py-24 px-6 md:px-12 bg-surface-container-lowest/50 border-t border-outline-variant/10">
        <div className="max-w-[1500px] mx-auto">
          {/* Asymmetrical Detail Row 1: Preflight */}
          <div className="flex flex-col lg:flex-row gap-16 items-center mb-32">
            <div className="flex-1 space-y-6">
              <span className="font-sans text-xs font-extrabold tracking-widest text-primary uppercase bg-primary/10 border border-primary/20 px-3.5 py-1.5 rounded-full inline-block">
                {t.usabilityEyebrow}
              </span>
              <h2 className="font-serif text-4xl text-text-primary font-medium tracking-tight leading-snug">
                {t.usabilityTitle}
              </h2>
              <p className="text-text-secondary text-base leading-relaxed">
                {t.usabilityDesc}
              </p>
            </div>

            <div className="flex-1 w-full relative">
              <div className="absolute -inset-3 bg-primary/5 rounded-2xl blur-xl"></div>
              <img
                src="./images/iphone-workflow.png?v=2"
                alt="Preflight diagnostics dashboard"
                referrerPolicy="no-referrer"
                className="rounded-2xl border border-outline-variant/30 shadow-2xl relative z-10 w-full"
              />
            </div>
          </div>

          {/* Asymmetrical Detail Row 2: Dark Halls */}
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-16 items-center">
            {/* Visual double thumbnail block */}
            <div className="lg:col-span-6 order-2 lg:order-1 grid grid-cols-2 gap-4">
              <div className="space-y-4">
                <img
                  src="./images/preflight.png?v=2"
                  alt="Sound asset details card"
                  referrerPolicy="no-referrer"
                  className="rounded-xl border border-outline-variant/20 shadow-md w-full"
                />
              </div>
              <div className="space-y-4 pt-8">
                <img
                  src="./images/dark-interface.png?v=2"
                  alt="Desktop arrangements board"
                  referrerPolicy="no-referrer"
                  className="rounded-xl border border-outline-variant/20 shadow-md w-full"
                />
              </div>
            </div>

            {/* Dark Hall description content */}
            <div className="lg:col-span-6 order-1 lg:order-2 space-y-6">
              <h2 className="font-serif text-4xl text-text-primary font-medium tracking-tight leading-snug">
                {t.darkHallTitle}
              </h2>
              <p className="text-text-secondary text-base leading-relaxed">
                {t.darkHallDesc}
              </p>

              <ul className="space-y-4 pt-2">
                <li className="flex items-center gap-3.5 text-text-muted text-sm font-medium">
                  <span className="w-5 h-5 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center text-primary text-xs font-bold">
                    ✓
                  </span>
                  <span>{t.darkHallBullet1}</span>
                </li>
                <li className="flex items-center gap-3.5 text-text-muted text-sm font-medium">
                  <span className="w-5 h-5 rounded-full bg-primary/10 border border-primary/20 flex items-center justify-center text-primary text-xs font-bold">
                    ✓
                  </span>
                  <span>{t.darkHallBullet2}</span>
                </li>
              </ul>
            </div>
          </div>
        </div>
      </section>

      {/* TOASTMASTER STORY / TESTIMONIAL BLOCK */}
      <section
        className="py-24 px-6 md:px-12 relative overflow-hidden bg-surface"
        id="story"
      >
        <div className="max-w-5xl mx-auto z-10 relative">
          <div className="bento-card rounded-3xl p-8 md:p-16 border border-primary/25 bg-gradient-to-br from-surface-container-high to-surface-container-lowest">
            <div className="flex flex-col items-center text-center">
              {/* Quote Circle Icon */}
              <div className="w-14 h-14 rounded-full bg-primary/10 border border-primary/30 flex items-center justify-center mb-6">
                <span className="font-serif text-primary text-2xl font-bold">
                  “
                </span>
              </div>

              <span className="font-sans text-xs font-extrabold tracking-widest text-primary uppercase block mb-4">
                {t.storyEyebrow}
              </span>

              <h2 className="font-serif text-3xl md:text-4xl text-text-primary font-medium tracking-tight mb-12 italic leading-tight">
                {t.storyQuote}
              </h2>

              {/* Grid detail content */}
              <div className="grid grid-cols-1 md:grid-cols-12 gap-10 text-left items-center">
                {/* Testimonial Quote paragraph */}
                <div className="md:col-span-7 space-y-6 text-sm text-text-secondary leading-relaxed">
                  <p className="italic">{t.storyPara1}</p>
                  <p className="italic">{t.storyPara2}</p>
                </div>

                {/* Simulated Floating Device frame */}
                <div className="md:col-span-5 space-y-4">
                  <div className="bg-[#11100e] p-2 rounded-[2rem] border-4 border-outline-variant/30 shadow-2xl max-w-[220px] mx-auto transform rotate-3 hover:rotate-0 transition-transform duration-500">
                    <img
                      src="./images/operator-story.png?v=2"
                      alt="Lydkontroll Mobile wedding cue"
                      referrerPolicy="no-referrer"
                      className="rounded-[1.75rem] w-full"
                    />
                  </div>
                  <p className="text-[10px] text-text-muted text-center font-bold tracking-wider uppercase">
                    {t.storyCaption}
                  </p>
                </div>
              </div>

              {/* Card Footer badges */}
              <div className="mt-12 pt-8 border-t border-outline-variant/15 w-full flex flex-col sm:flex-row items-center justify-between gap-6">
                <div className="text-center sm:text-left">
                  <h4 className="font-sans font-bold text-text-primary text-base">
                    {t.storyUserTitle}
                  </h4>
                  <p className="text-primary font-sans font-extrabold text-[10px] tracking-wider uppercase mt-0.5">
                    {t.storyUserSub}
                  </p>
                </div>

                <div className="flex gap-3">
                  <span className="px-3.5 py-1.5 bg-status-success/10 border border-status-success/30 rounded-full text-status-success text-xs font-bold flex items-center gap-1.5">
                    <span className="w-1.5 h-1.5 rounded-full bg-status-success"></span>
                    {t.badgeDowntime}
                  </span>
                  <span className="px-3.5 py-1.5 bg-status-success/10 border border-status-success/30 rounded-full text-status-success text-xs font-bold flex items-center gap-1.5">
                    <span className="w-1.5 h-1.5 rounded-full bg-status-success"></span>
                    {t.badgeLatency}
                  </span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </section>

      {/* ROBUSTNESS & SECURITY */}
      <section
        className="py-24 px-6 md:px-12 bg-surface-container-low border-y border-outline-variant/15"
        id="robustness"
      >
        <div className="max-w-[1500px] mx-auto">
          <div className="text-center mb-20">
            <h2 className="font-serif text-4xl md:text-5xl text-text-primary font-medium tracking-tight mb-4">
              {t.robustTitle}
            </h2>
            <p className="text-text-muted text-base max-w-2xl mx-auto">
              {t.robustSub}
            </p>
          </div>

          <div className="grid grid-cols-1 md:grid-cols-3 gap-8">
            {/* Fail-safe box 1 */}
            <div className="bg-surface-container p-8 rounded-2xl border border-outline-variant/10 text-center space-y-4">
              <div className="w-12 h-12 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary mx-auto">
                <Cpu size={24} />
              </div>
              <h3 className="font-serif text-xl text-text-primary font-medium">
                {t.robust1Title}
              </h3>
              <p className="text-text-muted text-sm leading-relaxed">
                {t.robust1Desc}
              </p>
            </div>

            {/* Fail-safe box 2 */}
            <div className="bg-surface-container p-8 rounded-2xl border border-outline-variant/10 text-center space-y-4">
              <div className="w-12 h-12 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary mx-auto">
                <Sliders size={24} />
              </div>
              <h3 className="font-serif text-xl text-text-primary font-medium">
                {t.robust2Title}
              </h3>
              <p className="text-text-muted text-sm leading-relaxed">
                {t.robust2Desc}
              </p>
            </div>

            {/* Fail-safe box 3 */}
            <div className="bg-surface-container p-8 rounded-2xl border border-outline-variant/10 text-center space-y-4">
              <div className="w-12 h-12 rounded-xl bg-primary/10 border border-primary/20 flex items-center justify-center text-primary mx-auto">
                <RefreshCw size={24} />
              </div>
              <h3 className="font-serif text-xl text-text-primary font-medium">
                {t.robust3Title}
              </h3>
              <p className="text-text-muted text-sm leading-relaxed">
                {t.robust3Desc}
              </p>
            </div>
          </div>

          {/* Technical specification details banner */}
          <div className="mt-16 p-6 rounded-2xl bg-surface border border-outline-variant/20 flex flex-col md:flex-row items-center gap-6">
            <div className="flex-shrink-0 flex items-center gap-2 px-4 py-2 bg-primary/10 rounded-full border border-primary/20">
              <Lock size={14} className="text-primary" />
              <span className="text-primary font-bold text-[10px] uppercase tracking-wider">
                {t.techDetails}
              </span>
            </div>
            <p className="text-text-secondary text-xs leading-relaxed">
              {t.techDesc}
            </p>
          </div>
        </div>
      </section>

      {/* OPEN SOURCE LICENSE BRIEFING */}
      <section className="py-24 px-6 md:px-12 bg-surface">
        <div className="max-w-4xl mx-auto bento-card rounded-3xl p-8 md:p-12 text-center border-primary/20 bg-gradient-to-b from-surface-container-high to-surface-container">
          <div className="w-12 h-12 rounded-full bg-primary/10 border border-primary/30 flex items-center justify-center mb-6 mx-auto text-primary">
            <HeartHandshake size={24} />
          </div>
          <h2 className="font-serif text-3xl text-text-primary font-medium mb-4">
            {t.licenseTitle}
          </h2>
          <p className="text-text-secondary text-sm md:text-base leading-relaxed mb-4">
            {t.licenseDesc}
          </p>
          <span className="text-xs font-mono text-primary font-bold bg-primary/10 border border-primary/20 px-3 py-1 rounded-full inline-block">
            PolyForm Noncommercial 1.0.0
          </span>
        </div>
      </section>

      {/* FINAL CALL TO ACTION */}
      <section className="py-32 bg-surface-container-lowest text-center relative overflow-hidden border-t border-outline-variant/15">
        <div className="absolute top-0 left-1/2 transform -translate-x-1/2 -translate-y-1/2 w-96 h-96 bg-primary/5 rounded-full blur-[100px] pointer-events-none"></div>

        <div className="relative z-10 px-6 max-w-2xl mx-auto space-y-8">
          <h2 className="font-serif text-4xl md:text-5xl italic font-normal text-text-primary leading-tight">
            {t.ctaTitle}
          </h2>
          <div className="flex justify-center">
            <a
              href="https://github.com/dagjomar/lydkontroll"
              target="_blank"
              rel="noreferrer"
              className="px-10 py-5 bg-primary text-on-primary font-bold rounded-xl text-lg flex items-center justify-center gap-3 transition-transform hover:scale-105 active:scale-98 shadow-xl shadow-primary/10"
            >
              <Github size={20} />
              <span>{t.ctaBtn}</span>
            </a>
          </div>
        </div>
      </section>

      {/* PROFESSIONAL FOOTER */}
      <footer className="w-full py-12 px-6 md:px-12 bg-surface-container-lowest max-w-[1500px] mx-auto border-t border-outline-variant/20">
        <div className="flex flex-col md:flex-row justify-between items-center gap-8">
          {/* Logo Sign */}
          <div className="flex flex-col items-center md:items-start gap-2">
            <span className="font-serif font-bold text-xl text-text-secondary opacity-80">
              Lydkontroll
            </span>
            <p className="text-text-muted text-xs text-center md:text-left leading-relaxed">
              {t.footerDesc}
            </p>
          </div>

          {/* Quick Links */}
          <div className="flex flex-wrap justify-center gap-x-8 gap-y-3">
            <a
              href="https://github.com/dagjomar/lydkontroll"
              target="_blank"
              rel="noreferrer"
              className="text-xs text-text-muted hover:text-text-primary font-bold tracking-wider uppercase transition-colors"
            >
              GitHub
            </a>
            <a
              href="#"
              className="text-xs text-text-muted hover:text-text-primary font-bold tracking-wider uppercase transition-colors"
            >
              DOKUMENTASJON
            </a>
            <a
              href="#"
              className="text-xs text-text-muted hover:text-text-primary font-bold tracking-wider uppercase transition-colors"
            >
              LISENS
            </a>
          </div>
        </div>
      </footer>
    </div>
  );
}
