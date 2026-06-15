use leptos::prelude::*;

use crate::components::{nail_bottle::NailBottle, swatch_strip::SwatchStrip};

/// Site footer — dark band with wordmark, location, hours, and nav.
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        // Swatch strip acts as page → footer transition
        <SwatchStrip />

        <footer class="bg-midnight-ink text-white">

            // ── Main content ──────────────────────────────────────────────
            <div class="page-container py-12 grid grid-cols-1 md:grid-cols-3 gap-10">

                // Col 1 — wordmark + tagline + CTA
                <div class="flex flex-col gap-5">
                    <a
                        href="/"
                        class="flex items-center gap-2 group w-fit"
                        aria-label="Colorful Nails & Spa — home"
                    >
                        <NailBottle class="w-4 h-10 text-blush-petal transition-transform duration-150 group-hover:-translate-y-1" />
                        <span class="font-display font-bold text-lg text-white leading-none">
                            "Colorful Nails & Spa"
                        </span>
                    </a>
                    <p class="font-sans text-sm text-white/50 max-w-[18rem]">
                        "Family-owned nail salon in Hazleton, PA. " "Open since 2011 and counting."
                    </p>
                    <a href="/booking" class="btn btn-primary btn-md btn-shimmer w-fit">
                        "Book Now"
                    </a>
                </div>

                // Col 2 — location + hours
                <div class="flex flex-col gap-4">
                    <p class="text-xs font-sans font-semibold tracking-widest uppercase text-white/30">
                        "Visit"
                    </p>
                    <a
                        rel="external"
                        href="https://maps.app.goo.gl/ZxRttxppY3V1qUxm8"
                        class="font-sans text-sm text-white/70 hover:text-white transition-colors leading-relaxed"
                    >
                        "546 W Broad St"
                        <br />
                        "Hazleton, PA 18201"
                        <br />
                        "Hazleton Shopping Center"
                    </a>
                    <table class="w-full font-sans text-sm" style="border-collapse: collapse">
                        <tbody>
                            <tr>
                                <td class="text-white/40 pr-4 py-1 font-medium">"Mon – Sat"</td>
                                <td class="text-white/70 tabular-nums">"9:30 AM – 7:30 PM"</td>
                            </tr>
                            <tr>
                                <td class="text-white/40 pr-4 py-1 font-medium">"Sunday"</td>
                                <td class="text-white/70 tabular-nums">"11:00 AM – 6:00 PM"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>

                // Col 3 — contact + links
                <div class="flex flex-col gap-4">
                    <p class="text-xs font-sans font-semibold tracking-widest uppercase text-white/30">
                        "Contact"
                    </p>
                    <a
                        href="tel:+15704552799"
                        class="font-sans text-sm text-white/70 hover:text-white transition-colors"
                    >
                        "(570) 455-2799"
                    </a>

                    <p class="text-xs font-sans font-semibold tracking-widest uppercase text-white/30 mt-2">
                        "Navigate"
                    </p>
                    <nav class="flex flex-col gap-2" aria-label="Footer navigation">
                        <a
                            href="/"
                            class="font-sans text-sm text-white/70 hover:text-white transition-colors w-fit"
                        >
                            "Home"
                        </a>
                        <a
                            href="/services"
                            class="font-sans text-sm text-white/70 hover:text-white transition-colors w-fit"
                        >
                            "Services"
                        </a>
                        <a
                            href="/booking"
                            class="font-sans text-sm text-white/70 hover:text-white transition-colors w-fit"
                        >
                            "Book an Appointment"
                        </a>
                    </nav>
                </div>

            </div>

            // ── Copyright bar ─────────────────────────────────────────────
            <div style="border-top: 1px solid rgba(255,255,255,0.08)">
                <div class="page-container py-4 flex flex-col sm:flex-row items-center justify-between gap-2">
                    <p class="font-sans text-xs text-white/30">
                        "© 2026 Colorful Nails & Spa · Hazleton, PA"
                    </p>
                    <p class="font-sans text-xs text-white/20">"Walk-ins welcome"</p>
                </div>
            </div>

        </footer>
    }
}
