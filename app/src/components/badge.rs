use leptos::prelude::*;

/// Inline status badge.
///
/// # Props
/// - `variant` — `"coral"` · `"violet"` · `"gold"` · `"teal"` · `"blush"` · `"mint"` · `"outline"` (default: `"outline"`)
/// - `class`   — Additional Tailwind classes.
///
/// # Examples
/// ```rust
/// <Badge variant="coral">"Confirmed"</Badge>
/// <Badge variant="mint">"Available"</Badge>
/// <Badge variant="gold">"Pending"</Badge>
/// <Badge variant="violet">"Gel"</Badge>
/// ```
#[component]
pub fn Badge(
    #[prop(optional, into)] variant: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant.as_str() {
        "coral" => "badge badge-coral",
        "violet" => "badge badge-violet",
        "gold" => "badge badge-gold",
        "teal" => "badge badge-teal",
        "blush" => "badge badge-blush",
        "mint" => "badge badge-mint",
        _ => "badge badge-outline",
    };

    let class = format!("{variant_class} {class}");

    view! {
        <span class=class>
            {children()}
        </span>
    }
}
