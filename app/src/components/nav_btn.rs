use leptos::prelude::*;

#[component]
pub fn NavButton<'a>(
    #[prop(default = "/")] href: &'a str,
    #[prop(default = "Navigate")] text_content: &'a str,
    #[prop(optional, default = "")] anchor_class: &'a str,
    #[prop(optional, default = "")] button_class: &'a str,
) -> impl IntoView {
    view! {
        <a href=href class=format!("{anchor_class}")>
            <button class=format!("{button_class}")>{text_content}</button>
        </a>
    }
}
