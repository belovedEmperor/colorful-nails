use leptos::prelude::*;

use crate::components::{
    alert::Alert,
    badge::Badge,
    button::Button,
    card::Card,
    empty_state::EmptyState,
    errors::ErrorView,
    input::{SelectInput, TextInput, Toggle},
    nail_bottle::{NailBottle, NailBottleRow},
    skeleton::{BookingCardSkeleton, ServiceRowSkeleton, TextSkeleton},
    swatch_strip::SwatchStrip,
};

// Palette: (name, css-color, tailwind-token, is-light-swatch)
// Defined as a const so hex values aren't inline in view! attribute syntax.
const PALETTE: &[(&str, &str, &str, bool)] = &[
    ("Coral Lacquer", "E8524A", "coral-lacquer", false),
    ("Violet Gloss", "7C3AED", "violet-gloss", false),
    ("Gold Shimmer", "F59E0B", "gold-shimmer", false),
    ("Teal Gel", "0891B2", "teal-gel", false),
    ("Blush Petal", "f9b4d7", "blush-petal", true),
    ("Mint Frost", "dcf9e0", "mint-frost", true),
    ("Champagne Fizz", "f6ffdb", "champagne-fizz", true),
    ("Powder Rose", "f5e4ef", "powder-rose", true),
    ("Midnight Ink", "050316", "midnight-ink", false),
    ("Porcelain", "fbfbfe", "porcelain", true),
];

// Bottle shelf variants for the design page size + dark-bg demos.
// Hex digits only — # prepended at call site so the scanner won't mistake
// these for secrets.
const BOTTLE_SIZES: &[(&str, &str)] = &[
    ("E8524A", "w-6 h-16"),
    ("7C3AED", "w-8 h-20"),
    ("F59E0B", "w-10 h-28"),
    ("0891B2", "w-12 h-32"),
    ("C084A8", "w-14 h-36"),
];
const BOTTLE_DARK: &[(&str, &str)] = &[
    ("E8524A", "w-8 h-20"),
    ("7C3AED", "w-8 h-24"),
    ("F59E0B", "w-8 h-18"),
    ("0891B2", "w-8 h-22"),
    ("C084A8", "w-8 h-20"),
    ("5AAF7A", "w-8 h-19"),
];

// ── Small layout helpers ───────────────────────────────────────────────────

#[component]
fn Section(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="section-padding">
            <div class="page-container flex flex-col gap-8">
                <h2 class="font-display text-2xl font-bold text-midnight-ink border-b border-midnight-ink/10 pb-3">
                    {title}
                </h2>
                {children()}
            </div>
        </section>
    }
}

#[component]
fn Row(#[prop(optional, into)] label: String, children: Children) -> impl IntoView {
    let has_label = !label.is_empty();
    view! {
        <div class="flex flex-col gap-2">
            <Show when=move || has_label>
                <p class="text-xs font-semibold font-sans uppercase tracking-widest text-midnight-ink/40">
                    {label.clone()}
                </p>
            </Show>
            <div class="flex flex-wrap items-center gap-3">
                {children()}
            </div>
        </div>
    }
}

// ── Color swatch ──────────────────────────────────────────────────────────

#[component]
fn Swatch(
    name: &'static str,
    color: &'static str,
    token: &'static str,
    #[prop(optional)] light: bool,
) -> impl IntoView {
    let border = if light {
        "border border-midnight-ink/10"
    } else {
        ""
    };
    let bg_style = format!("background-color: #{color}");
    let hex_label = format!("#{color}");

    view! {
        <div class="flex flex-col gap-1.5 w-32">
            <div
                class=format!("w-32 h-20 rounded-xl {border}")
                style=bg_style
            ></div>
            <div class="flex flex-col gap-0.5">
                <p class="text-xs font-semibold text-midnight-ink font-sans">{name}</p>
                <p class="text-xs font-mono text-midnight-ink/50">{hex_label}</p>
                <p class="text-xs font-mono text-midnight-ink/40">{token}</p>
            </div>
        </div>
    }
}

// ── Main design page ──────────────────────────────────────────────────────

/// Design system showcase — Polish Collection
#[component]
pub fn Design() -> impl IntoView {
    let toggle_a = RwSignal::new(false);
    let toggle_b = RwSignal::new(true);

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>

            // ── Page header ───────────────────────────────────────────────
            <div class="bg-midnight-ink text-white py-16">
                <div class="page-container flex flex-col md:flex-row items-center justify-between gap-8">
                    <div class="flex flex-col gap-2">
                        <p class="text-xs font-sans font-semibold tracking-widest uppercase text-white/40">
                            "Colorful Nails & Spa"
                        </p>
                        <h1 class="font-display text-title font-bold text-white">
                            "Polish Collection"
                        </h1>
                        <p class="font-sans text-sm text-white/60">
                            "Design system · Playfair Display + Inter · 10-token palette"
                        </p>
                    </div>
                    <NailBottleRow class="opacity-90" />
                </div>
            </div>

            // ── Palette ───────────────────────────────────────────────────
            <Section title="Palette">
                <p class="font-sans text-sm text-midnight-ink/60">
                    "Each color owns a semantic role and is never swapped freely. "
                    "Coral = action. Violet = browse. Gold = attention. Teal = links."
                </p>
                <div class="flex flex-wrap gap-6">
                    {PALETTE
                        .iter()
                        .map(|(name, color, token, light)| {
                            view! {
                                <Swatch
                                    name=name
                                    color=color
                                    token=token
                                    light=*light
                                />
                            }
                        })
                        .collect_view()}
                </div>
            </Section>

            <SwatchStrip />

            // ── Typography ────────────────────────────────────────────────
            <Section title="Typography">
                <Row label="Display — Playfair Display">
                    <div class="flex flex-col gap-2 w-full">
                        <p class="font-display font-bold" style="font-size: 3rem; line-height: 1.1">
                            "Colorful Nails & Spa"
                        </p>
                        <p class="font-display font-bold text-subtitle">
                            "Book Your Appointment"
                        </p>
                        <p class="font-display text-section font-semibold">
                            "Our Services"
                        </p>
                        <p class="font-display text-xl font-medium italic text-midnight-ink/70">
                            "Every shade tells a story."
                        </p>
                    </div>
                </Row>
                <Row label="Body — Inter">
                    <div class="flex flex-col gap-2 w-full font-sans">
                        <p class="text-base font-normal text-midnight-ink">
                            "Regular 16 — We've been in Hazleton for over 13 years. Family-owned, locally loved."
                        </p>
                        <p class="text-sm font-medium text-midnight-ink">
                            "Medium 14 — Open Monday through Saturday 9:30 AM – 7:30 PM, Sunday 11 AM – 6 PM."
                        </p>
                        <p class="text-xs font-semibold uppercase tracking-widest text-midnight-ink/50">
                            "Label 12 — Service category"
                        </p>
                        <p class="text-xs tabular-nums text-midnight-ink/60">
                            "Tabular nums — $25.00  $35.00  $45.00  $12.00"
                        </p>
                    </div>
                </Row>
            </Section>

            <SwatchStrip />

            // ── Nail Bottles ──────────────────────────────────────────────
            <Section title="Nail Bottle">
                <p class="font-sans text-sm text-midnight-ink/60">
                    "The brand's signature mark. Used decoratively — never as a functional icon."
                </p>
                <Row label="Single bottle — size variants">
                    {BOTTLE_SIZES
                        .iter()
                        .map(|(c, s)| {
                            let col = format!("#{c}");
                            view! { <NailBottle color=col class=*s /> }
                        })
                        .collect_view()}
                </Row>
                <Row label="NailBottleRow — shelf display">
                    <NailBottleRow />
                </Row>
                <Row label="On dark surface">
                    <div class="bg-midnight-ink rounded-xl p-6 flex gap-2 items-end">
                        {BOTTLE_DARK
                            .iter()
                            .map(|(c, s)| {
                                let col = format!("#{c}");
                                view! { <NailBottle color=col class=*s /> }
                            })
                            .collect_view()}
                    </div>
                </Row>
            </Section>

            <SwatchStrip />

            // ── Buttons ───────────────────────────────────────────────────
            <Section title="Buttons">
                <Row label="Variants">
                    <Button>"Primary"</Button>
                    <Button variant="secondary">"Secondary"</Button>
                    <Button variant="ghost">"Ghost"</Button>
                    <Button variant="danger">"Danger"</Button>
                </Row>
                <Row label="Sizes">
                    <Button size="sm">"Small"</Button>
                    <Button>"Medium"</Button>
                    <Button size="lg">"Large"</Button>
                </Row>
                <Row label="Shimmer — Book Now CTA only">
                    <Button shimmer=true size="lg">"Book Now"</Button>
                </Row>
                <Row label="Disabled">
                    <Button disabled=true>"Primary"</Button>
                    <Button variant="secondary" disabled=true>"Secondary"</Button>
                    <Button variant="ghost" disabled=true>"Ghost"</Button>
                </Row>
            </Section>

            <SwatchStrip />

            // ── Badges ────────────────────────────────────────────────────
            <Section title="Badges">
                <Row label="All variants">
                    <Badge variant="coral">"Confirmed"</Badge>
                    <Badge variant="violet">"Gel"</Badge>
                    <Badge variant="gold">"Pending"</Badge>
                    <Badge variant="teal">"Info"</Badge>
                    <Badge variant="blush">"New"</Badge>
                    <Badge variant="mint">"Available"</Badge>
                    <Badge variant="outline">"Outline"</Badge>
                </Row>
                <Row label="In context">
                    <div class="card flex items-center justify-between gap-4 w-full max-w-sm">
                        <div>
                            <p class="font-sans font-semibold text-sm text-midnight-ink">"Full Set Acrylic"</p>
                            <p class="font-sans text-xs text-midnight-ink/50 tabular-nums">"Friday · 2:00 PM"</p>
                        </div>
                        <Badge variant="coral">"Confirmed"</Badge>
                    </div>
                    <div class="card flex items-center justify-between gap-4 w-full max-w-sm">
                        <div>
                            <p class="font-sans font-semibold text-sm text-midnight-ink">"Pedicure"</p>
                            <p class="font-sans text-xs text-midnight-ink/50 tabular-nums">"Saturday · 11:00 AM"</p>
                        </div>
                        <Badge variant="gold">"Pending"</Badge>
                    </div>
                </Row>
            </Section>

            <SwatchStrip />

            // ── Cards ─────────────────────────────────────────────────────
            <Section title="Cards">
                <p class="font-sans text-sm text-midnight-ink/60">
                    "Border + shadow together, never one without the other. "
                    "Cards earn their place — used only when content has internal structure."
                </p>
                <Row label="Tint variants">
                    <Card class="w-40 h-24 flex items-center justify-center">
                        <span class="font-sans text-xs text-midnight-ink/50">"Default (white)"</span>
                    </Card>
                    <Card tint="blush" class="w-40 h-24 flex items-center justify-center">
                        <span class="font-sans text-xs text-midnight-ink/50">"Blush"</span>
                    </Card>
                    <Card tint="mint" class="w-40 h-24 flex items-center justify-center">
                        <span class="font-sans text-xs text-midnight-ink/50">"Mint"</span>
                    </Card>
                    <Card tint="champagne" class="w-40 h-24 flex items-center justify-center">
                        <span class="font-sans text-xs text-midnight-ink/50">"Champagne"</span>
                    </Card>
                    <Card tint="violet" class="w-40 h-24 flex items-center justify-center">
                        <span class="font-sans text-xs text-midnight-ink/50">"Violet"</span>
                    </Card>
                </Row>
                <Row label="Service card example">
                    <Card class="flex flex-col gap-3 max-w-xs w-full">
                        <div class="flex items-start justify-between gap-2">
                            <h3 class="font-display text-lg font-semibold text-midnight-ink">
                                "Full Set Gel"
                            </h3>
                            <Badge variant="violet">"Gel"</Badge>
                        </div>
                        <p class="font-sans text-sm text-midnight-ink/60">
                            "Long-lasting gel color applied over natural nails. Includes shape and cuticle care."
                        </p>
                        <div class="flex items-center justify-between mt-1">
                            <span class="font-sans font-bold text-midnight-ink tabular-nums">"$45"</span>
                            <Button size="sm" variant="secondary">"Book"</Button>
                        </div>
                    </Card>
                    <Card tint="blush" class="flex flex-col gap-3 max-w-xs w-full">
                        <div class="flex items-start justify-between gap-2">
                            <h3 class="font-display text-lg font-semibold text-midnight-ink">
                                "Classic Manicure"
                            </h3>
                            <Badge variant="blush">"Popular"</Badge>
                        </div>
                        <p class="font-sans text-sm text-midnight-ink/60">
                            "Shape, buff, cuticle care, and your choice of regular polish."
                        </p>
                        <div class="flex items-center justify-between mt-1">
                            <span class="font-sans font-bold text-midnight-ink tabular-nums">"$25"</span>
                            <Button size="sm">"Book"</Button>
                        </div>
                    </Card>
                </Row>
            </Section>

            <SwatchStrip />

            // ── Alerts ────────────────────────────────────────────────────
            <Section title="Alerts">
                <div class="flex flex-col gap-3 max-w-lg">
                    <Alert variant="success" title="Booking confirmed!">
                        "We'll see you Friday at 3:00 PM. Call us if anything changes."
                    </Alert>
                    <Alert variant="error" title="Something went wrong">
                        "We couldn't save your appointment. Try again or call "
                        <a class="link" href="tel:+15704552799">"(570) 455-2799"</a>
                        "."
                    </Alert>
                    <Alert variant="warning" title="Almost full">
                        "Only one slot left on Saturday afternoon. Book soon."
                    </Alert>
                    <Alert variant="info" title="Holiday hours">
                        "We're open Christmas Eve 10 AM – 4 PM. Closed Christmas Day."
                    </Alert>
                    <Alert variant="success">
                        "No title — just a short success message."
                    </Alert>
                </div>
            </Section>

            <SwatchStrip />

            // ── Form Controls ─────────────────────────────────────────────
            <Section title="Form Controls">
                <Row label="Text inputs">
                    <div class="flex flex-col gap-4 w-full max-w-sm">
                        <TextInput
                            id="demo-name"
                            label="Full name"
                            name="name"
                            placeholder="Jane Smith"
                        />
                        <TextInput
                            id="demo-phone"
                            label="Phone number"
                            input_type="tel"
                            name="phone"
                            placeholder="(570) 555-0100"
                            hint="We'll text a reminder 24 hours before."
                        />
                        <TextInput
                            id="demo-email"
                            label="Email"
                            input_type="email"
                            name="email"
                            placeholder="jane@example.com"
                            error="Please enter a valid email address."
                        />
                    </div>
                </Row>
                <Row label="Select">
                    <div class="w-full max-w-sm">
                        <SelectInput id="demo-service" label="Service" name="service">
                            <option value="">"Choose a service..."</option>
                            <option value="manicure">"Classic Manicure — $25"</option>
                            <option value="pedicure">"Classic Pedicure — $35"</option>
                            <option value="gel-mani">"Gel Manicure — $40"</option>
                            <option value="full-set">"Full Set Acrylic — $55"</option>
                            <option value="fill">"Acrylic Fill — $35"</option>
                        </SelectInput>
                    </div>
                </Row>
                <Row label="Toggle">
                    <div class="flex flex-col gap-3">
                        <Toggle
                            checked=toggle_a.read_only()
                            on_change=move |v| toggle_a.set(v)
                            label="Text reminders"
                            id="toggle-a"
                        />
                        <Toggle
                            checked=toggle_b.read_only()
                            on_change=move |v| toggle_b.set(v)
                            label="Email confirmation"
                            id="toggle-b"
                        />
                        <p class="font-sans text-xs text-midnight-ink/50">
                            "Text reminders: "
                            {move || if toggle_a.get() { "on" } else { "off" }}
                            " · Email: "
                            {move || if toggle_b.get() { "on" } else { "off" }}
                        </p>
                    </div>
                </Row>
                <Row label="Full booking form">
                    <Card class="w-full max-w-sm flex flex-col gap-4">
                        <h3 class="font-display text-lg font-semibold text-midnight-ink">
                            "Book an Appointment"
                        </h3>
                        <TextInput id="f-name" label="Name" name="name" placeholder="Jane Smith" />
                        <TextInput
                            id="f-phone"
                            label="Phone"
                            input_type="tel"
                            name="phone"
                            placeholder="(570) 555-0100"
                        />
                        <SelectInput id="f-service" label="Service" name="service">
                            <option value="">"Choose..."</option>
                            <option value="manicure">"Classic Manicure — $25"</option>
                            <option value="pedicure">"Classic Pedicure — $35"</option>
                            <option value="gel-mani">"Gel Manicure — $40"</option>
                        </SelectInput>
                        <TextInput
                            id="f-date"
                            label="Date & time"
                            input_type="datetime-local"
                            name="appointment_time"
                        />
                        <Button shimmer=true btn_type="submit" size="lg" class="w-full">
                            "Book Now"
                        </Button>
                    </Card>
                </Row>
            </Section>

            <SwatchStrip />

            // ── Skeletons ─────────────────────────────────────────────────
            <Section title="Skeletons">
                <p class="font-sans text-sm text-midnight-ink/60">
                    "Shaped like actual content — not generic gray rectangles. "
                    "The geometry tells you what's coming."
                </p>
                <Row label="Booking card skeleton">
                    <div class="w-full max-w-sm">
                        <BookingCardSkeleton />
                    </div>
                </Row>
                <Row label="Service list skeletons">
                    <div class="w-full max-w-sm card flex flex-col divide-y divide-midnight-ink/5">
                        <ServiceRowSkeleton />
                        <ServiceRowSkeleton />
                        <ServiceRowSkeleton />
                    </div>
                </Row>
                <Row label="Text skeleton">
                    <div class="w-full max-w-sm">
                        <TextSkeleton lines=4 />
                    </div>
                </Row>
            </Section>

            <SwatchStrip />

            // ── Empty State ───────────────────────────────────────────────
            <Section title="Empty State">
                <p class="font-sans text-sm text-midnight-ink/60">
                    "Always: title + reason + primary CTA. Never just \"No items yet.\""
                </p>
                <div class="card max-w-sm w-full mx-auto">
                    <EmptyState
                        title="No upcoming appointments"
                        description="You haven't booked anything yet. It only takes a minute."
                        action_href="/booking"
                        action_label="Book your first appointment"
                        icon="💅"
                    />
                </div>
            </Section>

            <SwatchStrip />

            // ── Design principles ─────────────────────────────────────────
            <Section title="Design Principles">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    <Card tint="blush" class="flex flex-col gap-2">
                        <p class="font-display text-base font-semibold text-midnight-ink">
                            "Color earns its name"
                        </p>
                        <p class="font-sans text-sm text-midnight-ink/70">
                            "Ten tokens, each with one semantic job. Coral books. Violet browses. Gold informs. Never decorating — always signaling."
                        </p>
                    </Card>
                    <Card tint="violet" class="flex flex-col gap-2">
                        <p class="font-display text-base font-semibold text-midnight-ink">
                            "One animation in the whole system"
                        </p>
                        <p class="font-sans text-sm text-midnight-ink/70">
                            "The shimmer sweep lives only on the Book Now button. Everything else is static. Restraint is what makes it land."
                        </p>
                    </Card>
                    <Card tint="champagne" class="flex flex-col gap-2">
                        <p class="font-display text-base font-semibold text-midnight-ink">
                            "Border and shadow together"
                        </p>
                        <p class="font-sans text-sm text-midnight-ink/70">
                            "Surface cards always have both. Border alone reads flat. Shadow alone reads soft. Together: a physical object on a surface."
                        </p>
                    </Card>
                    <Card tint="mint" class="flex flex-col gap-2">
                        <p class="font-display text-base font-semibold text-midnight-ink">
                            "Skeletons match content shape"
                        </p>
                        <p class="font-sans text-sm text-midnight-ink/70">
                            "A skeleton tells you what's coming. A generic gray box is a spinner in disguise. Every skeleton mirrors the real layout."
                        </p>
                    </Card>
                </div>
            </Section>

        </ErrorBoundary>
    }
}
