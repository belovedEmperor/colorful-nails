use leptos::prelude::*;

use crate::components::nail_bottle::NailBottle;

/// Top navigation header — sticky, white, with Book Now CTA.
#[component]
pub fn Header() -> impl IntoView {
    let menu_open = RwSignal::new(false);

    view! {
        <header class="bg-porcelain border-b border-midnight-ink/10 sticky top-0 z-50 shadow-sm">

            // ── Desktop bar ───────────────────────────────────────────────
            <div class="page-container h-16 flex items-center justify-between gap-6">

                // Logo: bottle mark + wordmark
                <a
                    href="/"
                    class="flex items-center gap-2 group shrink-0 focus-visible:outline-none"
                    aria-label="Colorful Nails & Spa — home"
                >
                    // NailBottle with no color prop → fill: currentColor,
                    // colored via text-coral-lacquer on the SVG element.
                    <NailBottle class="w-4 h-10 text-coral-lacquer transition-transform duration-150 group-hover:-translate-y-1" />
                    <span class="font-display font-bold text-lg text-midnight-ink leading-none">
                        "Colorful Nails"
                    </span>
                </a>

                // Desktop nav links
                <nav class="hidden md:flex items-center gap-8" aria-label="Main navigation">
                    <a
                        href="/"
                        class="font-sans text-sm font-medium text-midnight-ink/60 hover:text-midnight-ink transition-colors duration-150"
                    >
                        "Home"
                    </a>
                    <a
                        href="/services"
                        class="font-sans text-sm font-medium text-midnight-ink/60 hover:text-midnight-ink transition-colors duration-150"
                    >
                        "Services"
                    </a>
                    <a href="/booking" class="btn btn-primary btn-md">
                        "Book Now"
                    </a>
                </nav>

                // Mobile hamburger button
                <button
                    class="md:hidden p-2 -mr-2 text-midnight-ink rounded-lg focus-visible:outline-coral-lacquer"
                    aria-label=move || if menu_open.get() { "Close menu" } else { "Open menu" }
                    aria-expanded=move || menu_open.get()
                    on:click=move |_| menu_open.update(|o| *o = !*o)
                >
                    <Show
                        when=move || menu_open.get()
                        fallback=|| {
                            view! {
                                // Hamburger icon
                                <svg
                                    xmlns="http://www.w3.org/2000/svg"
                                    class="w-6 h-6"
                                    fill="none"
                                    viewBox="0 0 24 24"
                                    aria-hidden="true"
                                >
                                    <line
                                        x1="3"
                                        y1="6"
                                        x2="21"
                                        y2="6"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                    />
                                    <line
                                        x1="3"
                                        y1="12"
                                        x2="21"
                                        y2="12"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                    />
                                    <line
                                        x1="3"
                                        y1="18"
                                        x2="21"
                                        y2="18"
                                        stroke="currentColor"
                                        stroke-width="2"
                                        stroke-linecap="round"
                                    />
                                </svg>
                            }
                        }
                    >
                        // Close (×) icon
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="w-6 h-6"
                            fill="none"
                            viewBox="0 0 24 24"
                            aria-hidden="true"
                        >
                            <line
                                x1="6"
                                y1="6"
                                x2="18"
                                y2="18"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                            />
                            <line
                                x1="18"
                                y1="6"
                                x2="6"
                                y2="18"
                                stroke="currentColor"
                                stroke-width="2"
                                stroke-linecap="round"
                            />
                        </svg>
                    </Show>
                </button>

            </div>

            // ── Mobile dropdown ───────────────────────────────────────────
            <Show when=move || menu_open.get()>
                <nav
                    class="md:hidden bg-porcelain border-t border-midnight-ink/10 px-6 py-5 flex flex-col gap-1"
                    aria-label="Mobile navigation"
                >
                    <a
                        href="/"
                        class="font-sans text-sm font-medium text-midnight-ink py-3 border-b border-midnight-ink/8 hover:text-coral-lacquer transition-colors"
                        on:click=move |_| menu_open.set(false)
                    >
                        "Home"
                    </a>
                    <a
                        href="/services"
                        class="font-sans text-sm font-medium text-midnight-ink py-3 border-b border-midnight-ink/8 hover:text-coral-lacquer transition-colors"
                        on:click=move |_| menu_open.set(false)
                    >
                        "Services"
                    </a>
                    <a
                        href="/booking"
                        class="btn btn-primary btn-lg w-full justify-center mt-3"
                        on:click=move |_| menu_open.set(false)
                    >
                        "Book Now"
                    </a>
                </nav>
            </Show>

        </header>
    }
}
