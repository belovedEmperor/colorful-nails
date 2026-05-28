use crate::components::{errors::ErrorView, header::Header, nav_btn::NavButton};
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <Header />

            <div class="flex flex-col">
                <h1 class="text-6xl tracking-widest italic text-center py-16">
                    "Welcome to Colorful Nails & Spa"
                </h1>

                <section class="bg-green-200 p-24">
                    <div class="page-container flex flex-col justify-center items-center gap-24">
                        <h2 class="text-5xl font-bold tracking-wide">"Make An Appointment Now!"</h2>
                        <NavButton
                            class="text-2xl bg-pink-300 text-white font-semibold px-10 py-4 rounded"
                            href="/booking"
                            text_content="Book Now!"
                        />
                    </div>
                </section>

                <section class="bg-orange-200 p-24">
                    <div class="page-container flex flex-col justify-center items-center gap-16">
                        <h2 class="text-4xl font-bold tracking-wide">"Where Are We?"</h2>
                        <p class="text-xl">
                            "We're located at "
                            <a
                                href="https://maps.app.goo.gl/ZxRttxppY3V1qUxm8"
                                class="text-blue-600 underline hover:text-blue-800 visited:text-purple-800 hover:visited:text-purple-900"
                            >
                                "546 W Broad St, Hazleton, PA 18201"
                            </a> " in the "
                            <a
                                href="https://maps.app.goo.gl/YH4pRVnfSKjFmCuP9"
                                class="text-blue-600 underline hover:text-blue-800 visited:text-purple-800 hover:visited:text-purple-900"
                            >
                                "Hazleton Shopping Center!"
                            </a>
                        </p>
                        <iframe
                            src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3013.1537856252676!2d-75.98709772245651!3d40.95620977135797!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x89c5a473e6c6ba81%3A0x605e3e0385f455f2!2sColorful%20Nails%20%26%20Spa!5e0!3m2!1sen!2sus!4v1780006481155!5m2!1sen!2sus"
                            width="600"
                            height="450"
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
