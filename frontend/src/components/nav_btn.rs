use leptos::prelude::*;

#[component]
pub fn NavButton<'a>(
    #[prop(default = "/")] href: &'a str,
    #[prop(default = "Navigate")] text_content: &'a str,
) -> impl IntoView {
    view! {
        <button class="nav-button">
            <a href=href>{text_content}</a>
        </button>
    }
}
