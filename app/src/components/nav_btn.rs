use leptos::prelude::*;

#[component]
pub fn NavButton<'a>(
    #[prop(default = "/")] href: &'a str,
    #[prop(default = "Navigate")] text_content: &'a str,
    #[prop(optional, default = "")] class: &'a str,
) -> impl IntoView {
    view! {
        <a href=href>
            <button class=format!("{class}")>{text_content}</button>
        </a>
    }
}
