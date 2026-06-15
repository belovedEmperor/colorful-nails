use leptos::prelude::*;

/// General-purpose button component.
///
/// # Props
/// - `variant` — `"primary"` (default) · `"secondary"` · `"ghost"` · `"danger"`
/// - `size`    — `"sm"` · `"md"` (default) · `"lg"`
/// - `shimmer` — Enables the Book-Now shimmer animation. Use only on the main CTA.
/// - `disabled`— Disabled state (static).
/// - `btn_type`— HTML `type` attribute; defaults to `"button"`. Use `"submit"` in forms.
/// - `class`   — Additional Tailwind classes.
///
/// # Examples
/// ```rust
/// // Primary CTA (no shimmer — standard action button)
/// <Button>"Save appointment"</Button>
///
/// // THE Book Now shimmer CTA (one per page)
/// <Button shimmer=true btn_type="submit">"Book Now"</Button>
///
/// // Secondary browse action
/// <Button variant="secondary">"View services"</Button>
///
/// // Ghost cancel
/// <Button variant="ghost">"Cancel"</Button>
/// ```
#[component]
pub fn Button(
    #[prop(optional, into)] variant: String,
    #[prop(optional, into)] size: String,
    #[prop(optional)] shimmer: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] btn_type: String,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let variant_class = match variant.as_str() {
        "secondary" => "btn btn-secondary",
        "ghost" => "btn btn-ghost",
        "danger" => "btn btn-danger",
        _ => "btn btn-primary",
    };

    let shimmer_class = if shimmer { " btn-shimmer" } else { "" };

    let size_class = match size.as_str() {
        "sm" => " btn-sm",
        "lg" => " btn-lg",
        _ => " btn-md",
    };

    let btn_type = if btn_type.is_empty() {
        "button".to_owned()
    } else {
        btn_type
    };

    let class = format!("{variant_class}{shimmer_class}{size_class} {class}");

    view! {
        <button class=class disabled=disabled type=btn_type>
            {children()}
        </button>
    }
}
