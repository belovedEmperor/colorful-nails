use leptos::prelude::*;

#[component]
pub fn NavButton<'a>(
    #[prop(default = "/")] href: &'a str,
    #[prop(default = "Navigate")] text_content: &'a str,
    #[prop(optional, default = "")] class: &'a str,
) -> impl IntoView {
    view! {
        <button class=format!("{class}")>
            <a href=href>{text_content}</a>
        </button>
    }
}
