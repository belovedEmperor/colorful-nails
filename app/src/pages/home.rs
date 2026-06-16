use leptos::prelude::*;

use crate::components::{
    errors::ErrorView, nail_bottle::NailBottleRow, nav_btn::NavButton, swatch_strip::SwatchStrip,
};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="flex flex-col">
                <section class="relative section-padding py-48 bg-[url('/logo.jpg')] bg-cover bg-center">
                    <div class="absolute inset-0 bg-white/80 pointer-events-none"></div>
                    <div class="relative z-10 page-container flex flex-row items-center gap-10 md:gap-16">

                        // Text side
                        <div class="flex flex-col gap-5 flex-1 items-center md:items-start text-center md:text-left">
                            <h1 class="font-display text-title font-bold text-midnight-ink">
                                "Colorful Nails & Spa"
                            </h1>
                            <p class="text-body text-midnight-ink/70 max-w-sm">
                                "Family-owned nail salon in Hazleton, PA." <br />
                                "Open for over 16 years."
                            </p>
                            <NavButton
                                button_class="btn btn-primary btn-lg"
                                href="/booking"
                                text_content="Book Now"
                            />
                        </div>

                        // Bottle shelf — decorative, aria-hidden inside NailBottleRow
                        <div class="md:flex justify-center md:justify-end flex-1 hidden">
                            <NailBottleRow class="py-4" />
                        </div>

                    </div>
                </section>

                <SwatchStrip />

                // ── Who Are We ─────────────────────────────────────────────
                <section class="section-padding section-container page-container flex flex-col">
                    <h2 class="section-header">"Who Are We?"</h2>
                    <p>
                        "We're a family-owned nail salon that's been in Hazleton for more than 16 years!"
                    </p>
                </section>

                <SwatchStrip />

                // ── Hours ──────────────────────────────────────────────────
                <section class="section-padding section-container page-container flex flex-col">
                    <h2 class="section-header">"When Are We Open?"</h2>
                    <table class="hours-table">
                        <tbody>
                            <tr>
                                <td>"Monday"</td>
                                <td>"9:30 AM – 7:30 PM"</td>
                            </tr>
                            <tr>
                                <td>"Tuesday"</td>
                                <td>"9:30 AM – 7:30 PM"</td>
                            </tr>
                            <tr>
                                <td>"Wednesday"</td>
                                <td>"9:30 AM – 7:30 PM"</td>
                            </tr>
                            <tr>
                                <td>"Thursday"</td>
                                <td>"9:30 AM – 7:30 PM"</td>
                            </tr>
                            <tr>
                                <td>"Friday"</td>
                                <td>"9:30 AM – 7:30 PM"</td>
                            </tr>
                            <tr>
                                <td>"Saturday"</td>
                                <td>"9:30 AM – 7:30 PM"</td>
                            </tr>
                            <tr>
                                <td>"Sunday"</td>
                                <td>"11:00 AM – 6:00 PM"</td>
                            </tr>
                        </tbody>
                    </table>
                    <p>
                        "We're often open on holidays when other stores aren't. Please "
                        <a class="link" href="tel:+15704552799">
                            "call us"
                        </a> " to check."
                    </p>
                </section>

                <SwatchStrip />

                // ── Location ───────────────────────────────────────────────
                <section class="section-padding section-container page-container flex flex-col">
                    <h2 class="section-header">"Where Are We?"</h2>
                    <p>
                        "We're at "
                        <a
                            rel="external"
                            href="https://maps.app.goo.gl/ZxRttxppY3V1qUxm8"
                            class="link"
                        >
                            "546 W Broad St, Hazleton, PA 18201"
                        </a> " in the "
                        <a
                            rel="external"
                            href="https://maps.app.goo.gl/YH4pRVnfSKjFmCuP9"
                            class="link"
                        >
                            "Hazleton Shopping Center"
                        </a> "."
                    </p>
                    <iframe
                        class="w-full max-w-2xl h-96 mx-auto rounded-xl border"
                        src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3013.1537856252676!2d-75.98709772245651!3d40.95620977135797!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x89c5a473e6c6ba81%3A0x605e3e0385f455f2!2sColorful%20Nails%20%26%20Spa!5e0!3m2!1sen!2sus!4v1780006481155!5m2!1sen!2sus"
                        style="border:0;"
                        allowfullscreen=""
                        loading="lazy"
                        referrerpolicy="no-referrer-when-downgrade"
                    ></iframe>
                </section>

                <SwatchStrip />

                // ── Contact ────────────────────────────────────────────────
                <section class="section-padding section-container page-container flex flex-col">
                    <h2 class="section-header">"Call Us"</h2>
                    <p>
                        "Questions? Call us at " <a class="link" href="tel:+15704552799">
                            "(570) 455-2799"
                        </a> "."
                    </p>
                </section>

            </div>
        </ErrorBoundary>
    }
}
