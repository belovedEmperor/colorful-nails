use leptos::prelude::*;

use crate::components::{errors::ErrorView, nav_btn::NavButton};

/// Header
#[component]
pub fn Header() -> impl IntoView {
    let menu_open = RwSignal::new(false);

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="bg-primary min-h-20 relative z-50">
                <div class="page-container h-20 items-center justify-between flex">
                    <a href="/">
                        <img
                            class="font-bold text-ink"
                            src="logo.jpg"
                            alt="Temporary Image"
                            width=50
                            height=50
                        />
                    </a>
                    <button
                        class="md:hidden text-section"
                        on:click=move |_| menu_open.update(|open| *open = !*open)
                    >
                        "☰"
                    </button>
                    <nav class="gap-12 font-bold text-ink hidden md:flex items-center">
                        <NavButton href="/" text_content="Home" />
                        <NavButton href="/services" text_content="Services" />
                        <NavButton
                            button_class="font-bold text-ink bg-secondary button px-5 py-2"
                            href="/booking"
                            text_content="Booking"
                        />
                    </nav>
                </div>
                <Show when=move || menu_open.get()>
                    <div class="bg-primary p-8 flex flex-col">
                        <NavButton
                            anchor_class="p-2 w-full text-center"
                            href="/"
                            text_content="Home"
                        />
                        <NavButton
                            anchor_class="p-2 w-full text-center"
                            href="/services"
                            text_content="Services"
                        />
                        <NavButton
                            anchor_class="p-2 w-full text-center"
                            href="/booking"
                            text_content="Booking"
                        />
                    </div>
                </Show>
            </div>
        </ErrorBoundary>
    }
}
