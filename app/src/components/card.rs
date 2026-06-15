use leptos::prelude::*;

/// Surface card — border + shadow together, never one without the other.
///
/// # Props
/// - `tint`  — Optional tinted surface: `"blush"` · `"mint"` · `"champagne"` · `"violet"`.
///             Default: white.
/// - `class` — Additional Tailwind classes (padding, width, etc.).
///
/// # Examples
/// ```rust
/// // Default white card
/// <Card>
///     <h3>"My Appointment"</h3>
///     <p>"Friday, 3:00 PM"</p>
/// </Card>
///
/// // Tinted
/// <Card tint="blush">
///     <p>"Special offer"</p>
/// </Card>
/// ```
#[component]
pub fn Card(
    #[prop(optional, into)] tint: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let tint_class = match tint.as_str() {
        "blush" => " card-blush",
        "mint" => " card-mint",
        "champagne" => " card-champagne",
        "violet" => " card-violet",
        _ => "",
    };

    let class = format!("card{tint_class} {class}");

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
