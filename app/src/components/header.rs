use leptos::prelude::*;

use crate::components::{errors::ErrorView, nav_btn::NavButton};

/// Header
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="bg-primary h-20">
                <div class="page-container h-full items-center justify-between grid grid-cols-[1fr_auto_1fr] grid-rows-1">
                    <img
                        class="font-bold text-ink"
                        src="logo.jpg"
                        alt="Temporary Image"
                        width=50
                        height=50
                    />
                    <nav class="space-x-12 font-bold text-ink">
                        <NavButton href="/" text_content="Home" />
                        <NavButton href="/services" text_content="Services" />
                    </nav>
                    <nav class="ml-auto">
                        <NavButton
                            button_class="font-bold text-ink bg-secondary button px-5 py-2"
                            href="/booking"
                            text_content="Booking"
                        />
                    </nav>
                </div>
            </div>
        </ErrorBoundary>
    }
}
