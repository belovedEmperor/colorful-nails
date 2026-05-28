use leptos::prelude::*;

use crate::components::{errors::ErrorView, nav_btn::NavButton};

/// Header
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="bg-pink-400 h-20 rounded-b-2xl">
                <div class="page-container h-full items-center justify-between grid grid-cols-[1fr_auto_1fr] grid-rows-1">
                    <img
                        class="text-xl font-bold text-gray-800"
                        src="../../public/favicon.ico"
                        alt="Temporary Image"
                    />
                    <nav class="grid grid-cols-2 gap-8 text-xl font-bold text-gray-800">
                        <NavButton href="/" text_content="Home" />
                        <NavButton href="/services" text_content="Services" />
                    </nav>
                    <nav class="ml-auto text-xl font-bold text-gray-800 bg-green-200 rounded-lg px-5 py-2">
                        <NavButton href="/booking" text_content="Booking" />
                    </nav>
                </div>
            </div>
        </ErrorBoundary>
    }
}
