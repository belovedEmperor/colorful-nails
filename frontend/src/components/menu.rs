use leptos::prelude::*;

use crate::components::nav_btn::NavButton;

/// Default Home Page
#[component]
pub fn Menu() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">
                <div class="header">
                    <img src="../../public/favicon.ico" alt="Temporary Image" />
                    <nav class="menu-options">
                        <NavButton href="/" text_content="Home" />
                        <NavButton href="/services" text_content="Services" />
                    </nav>
                    <nav class="menu-options">
                        <NavButton href="/booking" text_content="Booking" />
                    </nav>
                </div>
            </div>
        </ErrorBoundary>
    }
}
