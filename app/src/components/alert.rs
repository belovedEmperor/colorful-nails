use leptos::prelude::*;

/// Inline alert / feedback banner.
///
/// # Props
/// - `variant` — `"success"` · `"error"` · `"warning"` · `"info"` (default: `"info"`)
/// - `title`   — Optional bold heading inside the alert.
/// - `class`   — Additional Tailwind classes.
///
/// # Examples
/// ```rust
/// <Alert variant="success" title="Booking confirmed!">
///     "We'll see you Friday at 3:00 PM."
/// </Alert>
///
/// <Alert variant="error" title="Something went wrong">
///     "We couldn't save your appointment. Please try again or call (570) 455-2799."
/// </Alert>
/// ```
#[component]
pub fn Alert(
    #[prop(optional, into)] variant: String,
    #[prop(optional, into)] title: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let (variant_class, icon) = match variant.as_str() {
        "success" => ("alert alert-success", "✓"),
        "error" => ("alert alert-error", "✕"),
        "warning" => ("alert alert-warning", "⚠"),
        _ => ("alert alert-info", "ℹ"),
    };

    let class = format!("{variant_class} {class}");
    let has_title = !title.is_empty();

    view! {
        <div class=class role="alert">
            <span class="alert-icon">{icon}</span>
            <div>
                <Show when=move || has_title>
                    <p class="alert-title">{title.clone()}</p>
                </Show>
                <div class="alert-body">
                    {children()}
                </div>
            </div>
        </div>
    }
}
