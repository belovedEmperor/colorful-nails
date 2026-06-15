use leptos::prelude::*;

/// Complete empty state: title + reason + primary CTA.
/// Never show just "No items." — always give the user a next step.
///
/// # Props
/// - `title`       — Short heading, e.g. `"No appointments yet"`.
/// - `description` — One sentence explaining why and what to do.
/// - `action_href` — Where the CTA button links to.
/// - `action_label`— CTA button label, e.g. `"Book your first appointment"`.
/// - `icon`        — Optional emoji or small SVG string shown above the title.
///
/// # Example
/// ```rust
/// <EmptyState
///     title="No upcoming appointments"
///     description="You haven't booked anything yet. It only takes a minute."
///     action_href="/booking"
///     action_label="Book now"
///     icon="💅"
/// />
/// ```
#[component]
pub fn EmptyState(
    #[prop(into)] title: String,
    #[prop(into)] description: String,
    #[prop(into)] action_href: String,
    #[prop(into)] action_label: String,
    #[prop(optional, into)] icon: String,
) -> impl IntoView {
    let has_icon = !icon.is_empty();

    view! {
        <div class="flex flex-col items-center text-center py-16 px-6 gap-4">
            <Show when=move || has_icon>
                <span class="text-4xl mb-2" aria-hidden="true">
                    {icon.clone()}
                </span>
            </Show>
            <h3 class="font-display text-xl font-semibold text-midnight-ink">
                {title}
            </h3>
            <p class="text-sm text-midnight-ink/60 max-w-xs font-sans">
                {description}
            </p>
            <a
                href=action_href
                class="btn btn-primary btn-md btn-shimmer mt-2"
            >
                {action_label}
            </a>
        </div>
    }
}
