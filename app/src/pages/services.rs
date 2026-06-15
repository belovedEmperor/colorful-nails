use leptos::prelude::*;

use crate::components::{
    alert::Alert, card::Card, errors::ErrorView, nail_bottle::NailBottle, swatch_strip::SwatchStrip,
};

// ── Data ──────────────────────────────────────────────────────────────────

pub struct ServiceCategory {
    pub name: &'static str,
    pub services: &'static [&'static str],
    /// Badge / accent variant — matches CSS badge-* and card-* tint names.
    pub accent: &'static str,
    /// Card tint — pass to <Card tint=...>. Empty = white card.
    pub tint: &'static str,
    /// Hex color digits (no #) for the decorative `NailBottle` in each card.
    pub bottle: &'static str,
}

pub const SERVICE_CATEGORIES: &[ServiceCategory] = &[
    ServiceCategory {
        name: "Manicure & Pedicure",
        services: &["Manicure", "Gel Manicure", "Pedicure"],
        accent: "blush",
        tint: "blush",
        bottle: "C084A8",
    },
    ServiceCategory {
        name: "Nail Extras",
        services: &["Nail Repair", "Nail Trim", "Nail Design", "Nail Removal"],
        accent: "gold",
        tint: "champagne",
        bottle: "D4A017",
    },
    ServiceCategory {
        name: "Acrylic",
        services: &[
            "Full Set",
            "Fill-ins",
            "Pink & White",
            "Fill-ins Pink & White",
        ],
        accent: "violet",
        tint: "violet",
        bottle: "7C3AED",
    },
    ServiceCategory {
        name: "UV Gel",
        services: &["UV Gel", "UV Gel Fill-ins", "UV Gel Pink & White"],
        accent: "teal",
        tint: "",
        bottle: "0891B2",
    },
    ServiceCategory {
        name: "Polish",
        services: &["French", "Hands Polish Change", "Toes Polish Change"],
        accent: "coral",
        tint: "",
        bottle: "E8524A",
    },
    ServiceCategory {
        name: "Waxing",
        services: &[
            "Eyebrow Wax",
            "Lip Wax",
            "Arm Wax",
            "Half Arm Wax",
            "Leg Wax",
            "Underarm",
            "Bikini Wax",
            "Brazilian Wax",
        ],
        accent: "mint",
        tint: "mint",
        bottle: "5AAF7A",
    },
];

// ── Components ────────────────────────────────────────────────────────────

/// Single service category card.
#[component]
fn CategoryCard(category: &'static ServiceCategory) -> impl IntoView {
    let bottle_color = format!("#{}", category.bottle);

    view! {
        <Card tint=category.tint class="flex flex-col gap-4 h-full">
            // Header: name + bottle mark
            <div class="flex items-start justify-between gap-3">
                <h2 class="font-display text-lg font-bold text-midnight-ink leading-snug">
                    {category.name}
                </h2>
                <NailBottle color=bottle_color class="w-5 h-12 shrink-0 opacity-90" />
            </div>
            // Hairline
            <div class="h-px" style="background: rgba(5,3,22,0.08)"></div>
            // Service list
            <ul class="flex flex-col">
                {category
                    .services
                    .iter()
                    .map(|s| {
                        view! {
                            <li class="py-2.5 font-sans text-sm text-midnight-ink border-b border-midnight-ink/8 last:border-0">
                                {*s}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </Card>
    }
}

/// Waxing card — full width, services in a 2-column grid on larger screens.
#[component]
fn WaxingCard(category: &'static ServiceCategory) -> impl IntoView {
    let bottle_color = format!("#{}", category.bottle);

    view! {
        <Card tint=category.tint class="flex flex-col gap-4">
            <div class="flex items-start justify-between gap-3">
                <h2 class="font-display text-lg font-bold text-midnight-ink leading-snug">
                    {category.name}
                </h2>
                <NailBottle color=bottle_color class="w-5 h-12 shrink-0 opacity-90" />
            </div>
            <div class="h-px" style="background: rgba(5,3,22,0.08)"></div>
            <ul class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-4">
                {category
                    .services
                    .iter()
                    .map(|s| {
                        view! {
                            <li class="py-2.5 font-sans text-sm text-midnight-ink border-b border-midnight-ink/8 pr-4">
                                {*s}
                            </li>
                        }
                    })
                    .collect_view()}
            </ul>
        </Card>
    }
}

// ── Page ─────────────────────────────────────────────────────────────────

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>

            // ── Hero ──────────────────────────────────────────────────────
            <div class="bg-midnight-ink text-white py-16">
                <div class="page-container flex flex-col gap-3">
                    <p class="text-xs font-sans font-semibold tracking-widest uppercase text-white/40">
                        "Colorful Nails & Spa · Hazleton, PA"
                    </p>
                    <h1 class="font-display text-title font-bold text-white">"Our Services"</h1>
                    <p class="font-sans text-sm text-white/60 max-w-sm">
                        "Manicures, pedicures, acrylics, UV gel, waxing, and more."
                    </p>
                </div>
            </div>

            <SwatchStrip />

            // ── Pricing note ──────────────────────────────────────────────
            <div class="page-container">
                <Alert variant="info" title="Gel polish">
                    "Gel polish is an additional charge on any service. "
                    "Call us at "
                    <a class="link" href="tel:+15704552799">
                        "(570) 455-2799"
                    </a>
                    " for current pricing."
                </Alert>
            </div>

            // ── Section 1: manicure.jpg left, nail service cards right ────
            <section class="section-padding">
                <div class="page-container grid md:grid-cols-[1fr_1.2fr] gap-6 items-start">
                    <img
                        src="/manicure.jpg"
                        alt="Autumn nail art by Colorful Nails & Spa"
                        class="w-full rounded-xl object-cover md:sticky md:top-24"
                        style="aspect-ratio: 4/3"
                    />
                    <div class="flex flex-col gap-4">
                        <CategoryCard category=&SERVICE_CATEGORIES[0] />
                        <CategoryCard category=&SERVICE_CATEGORIES[1] />
                    </div>
                </div>
            </section>

            // ── Section 2: acrylic cards left, acrylic.jpg right ──────────
            <section class="section-padding">
                <div class="page-container grid md:grid-cols-[1.2fr_1fr] gap-6 items-start">
                    <div class="flex flex-col gap-4">
                        <CategoryCard category=&SERVICE_CATEGORIES[2] />
                        <CategoryCard category=&SERVICE_CATEGORIES[3] />
                        <CategoryCard category=&SERVICE_CATEGORIES[4] />
                    </div>
                    <img
                        src="/acrylic.jpg"
                        alt="3D acrylic nail art by Colorful Nails & Spa"
                        class="w-full rounded-xl object-cover md:sticky md:top-24"
                        style="aspect-ratio: 4/3"
                    />
                </div>
            </section>

            // ── Waxing — full width ────────────────────────────────────────
            <section class="section-padding pt-0">
                <div class="page-container">
                    <WaxingCard category=&SERVICE_CATEGORIES[5] />
                </div>
            </section>

            <SwatchStrip />

            // ── Bottom CTA ────────────────────────────────────────────────
            <section class="section-padding">
                <div class="page-container flex flex-col md:flex-row items-center justify-between gap-6">
                    <div class="flex flex-col gap-1 text-center md:text-left">
                        <h2 class="font-display text-section font-bold text-midnight-ink">
                            "Ready for your next appointment?"
                        </h2>
                        <p class="font-sans text-sm text-midnight-ink/60">
                            "Walk-ins welcome. Appointments get priority."
                        </p>
                    </div>
                    <a href="/booking" class="btn btn-primary btn-lg shrink-0">
                        "Book Now"
                    </a>
                </div>
            </section>

        </ErrorBoundary>
    }
}
