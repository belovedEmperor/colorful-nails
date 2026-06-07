use crate::components::{errors::ErrorView, nav_btn::NavButton};
use leptos::prelude::*;

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
                    <div class="relative z-10 page-container section-container flex flex-col items-center">
                        <div class="flex flex-col gap-2">
                            <h1 class="page-header">"Colorful Nails"</h1>
                            <h2 class="text-center text-subtitle">"Your Favorite Nail Salon"</h2>
                        </div>
                        <NavButton
                            button_class="bg-primary button"
                            href="/booking"
                            text_content="make an appointment"
                        />
                    </div>
                </section>

                <section>
                    <div class="section-padding section-container page-container flex flex-col justify-center section-container">
                        <h2 class="section-header">"Who Are We?"</h2>
                        <p>
                            "We're a nail salon that's been open for more than 13 years! If you have any questions, you can reach us at "
                            <a class="link" href="tel:+15704552799">
                                (570) 455-2799
                            </a>"."
                        </p>

                        <p>
                            "We're located at "
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
                            class="w-full max-w-2xl h-96 mx-auto"
                            src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3013.1537856252676!2d-75.98709772245651!3d40.95620977135797!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x89c5a473e6c6ba81%3A0x605e3e0385f455f2!2sColorful%20Nails%20%26%20Spa!5e0!3m2!1sen!2sus!4v1780006481155!5m2!1sen!2sus"
                            style="border:0;"
                            allowfullscreen=""
                            loading="lazy"
                            referrerpolicy="no-referrer-when-downgrade"
                        ></iframe>
                    </div>
                </section>
            </div>
        </ErrorBoundary>
    }
}
