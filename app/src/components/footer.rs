use leptos::prelude::*;

use crate::components::errors::ErrorView;
use crate::components::nav_btn::NavButton;

/// Footer
#[component]
pub fn Footer() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="bg-primary min-h-20">
                <div class="page-container h-20 items-center grid grid-rows-2">
                    <div class="grid grid-cols-3 w-full">
                        <p class="text-left">"Colorful Nails & Spa"<br /></p>
                        <a
                            rel="external"
                            href="https://maps.app.goo.gl/ZxRttxppY3V1qUxm8"
                            class="link text-center"
                        >
                            "546 W Broad St, Hazleton, PA 18201"
                        </a>
                        <a class="link text-right" href="tel:+15704552799">
                            "(570) 455-2799"
                        </a>
                    </div>
                    <p class="text-center">"© 2026 Colorful Nails & Spa"</p>
                </div>
            </div>
        </ErrorBoundary>
    }
}
