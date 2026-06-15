use leptos::prelude::*;

use crate::components::nail_bottle::NailBottle;

/// Signature section divider — six nail bottles flanked by hairline rules.
/// Used between major sections; nothing else gets this treatment.
///
/// Appears as:  ──────── 🧴🧴🧴🧴🧴🧴 ────────
#[component]
pub fn SwatchStrip() -> impl IntoView {
    // Hex digits only — # prepended at call site.
    const BOTTLES: &[&str] = &["E8524A", "7C3AED", "F59E0B", "0891B2", "C084A8", "5AAF7A"];

    view! {
        <div class="swatch-strip" aria-hidden="true">
            <div class="swatch-strip-rule"></div>
            <div class="flex items-end gap-1.5">
                {BOTTLES
                    .iter()
                    .map(|c| {
                        let color = format!("#{c}");
                        view! { <NailBottle color=color class="w-4 h-10" /> }
                    })
                    .collect_view()}
            </div>
            <div class="swatch-strip-rule"></div>
        </div>
    }
}
