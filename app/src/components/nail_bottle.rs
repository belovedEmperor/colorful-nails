use leptos::prelude::*;

/// The nail polish bottle SVG path. viewBox="245 82 93 252"
const BOTTLE_PATH: &str = "M332.7,323.2V200.3c0-2.7-2.2-4.9-4.9-4.9h-15.4v-4h2.4c2.7,0,4.9-2.2,4.9-4.9V88.7c0-2.7-2.2-4.9-4.9-4.9h-54.2c-2.7,0-4.9,2.2-4.9,4.9v97.8c0,2.7,2.2,4.9,4.9,4.9h1.6v4h-14.5c-2.7,0-4.9,2.2-4.9,4.9v122.9c0,2.7,2.2,4.9,4.9,4.9h80C330.5,328,332.7,325.9,332.7,323.2z M265.6,93.6h44.5v88.1h-44.5V93.6z M302.8,191.4v4h-30.7v-4H302.8z M323,318.3h-70.3V205.2H283v56.3l-4.4,33.6c-0.2,1.4,0.2,2.8,1.2,3.8c0.9,1,2.2,1.7,3.6,1.7h10c1.4,0,2.8-0.6,3.7-1.7c0.9-1.1,1.3-2.5,1.1-3.9l-5.5-33.6v-56.2H323V318.3z";

/// A single nail polish bottle rendered as an inline SVG.
///
/// # Props
/// - `color`  — CSS fill color string, e.g. `"#E8524A"` or `"currentColor"` (default).
/// - `class`  — Tailwind sizing / spacing, e.g. `"w-10 h-28"`.
/// - `label`  — Accessible label; leave empty for purely decorative use.
#[component]
pub fn NailBottle(
    #[prop(optional, into)] color: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] label: String,
) -> impl IntoView {
    let fill = if color.is_empty() {
        "currentColor".to_owned()
    } else {
        color
    };

    let aria_hidden = label.is_empty();
    let aria_label = if label.is_empty() { None } else { Some(label) };

    view! {
        <svg
            xmlns="http://www.w3.org/2000/svg"
            viewBox="245 82 93 252"
            class=class
            aria-hidden=aria_hidden
            aria-label=aria_label
            role=if aria_hidden { "none" } else { "img" }
        >
            <path d=BOTTLE_PATH fill=fill fill-rule="evenodd" />
        </svg>
    }
}

/// A decorative row of six nail polish bottles in the full palette.
/// Drop into any section as a hero accent or decorative element.
///
/// # Props
/// - `class` — Tailwind classes on the wrapper `<div>`, e.g. `"my-8"`.
#[component]
pub fn NailBottleRow(#[prop(optional, into)] class: String) -> impl IntoView {
    // Each (hex, scale) pair — varied heights give a natural "shelf" look.
    let bottles: &[(&str, &str)] = &[
        ("#E8524A", "w-8 h-20"), // coral-lacquer
        ("#7C3AED", "w-8 h-24"), // violet-gloss   — tallest
        ("#F59E0B", "w-8 h-18"), // gold-shimmer
        ("#0891B2", "w-8 h-22"), // teal-gel
        ("#C084A8", "w-8 h-20"), // blush-petal (slightly deepened for visibility)
        ("#5AAF7A", "w-8 h-19"), // mint-frost  (slightly deepened for visibility)
    ];

    let wrapper = format!("flex items-end gap-2 {class}");

    view! {
        <div class=wrapper aria-hidden="true">
            {bottles
                .iter()
                .map(|(color, size_class)| {
                    view! { <NailBottle color=*color class=*size_class /> }
                })
                .collect_view()}
        </div>
    }
}
