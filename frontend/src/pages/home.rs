use crate::components::{menu::Menu, nav_btn::NavButton};
use leptos::prelude::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
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
                <Menu />

                <img
                    src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg"
                    alt="Leptos Logo"
                    height="200"
                    width="400"
                />

                <h1>"Welcome to Colorful Nails & Spa"</h1>

                <section>
                    <h2>"Make an Appointment"</h2>
                    <NavButton href="/booking" text_content="Book Now!" />
                </section>

                <section>
                    <h2>"Where Are We?"</h2>
                    <p>
                        "We're located at 546 W Broad St, Hazleton, PA 18201 in the Hazleton Shopping"
                        Center!
                    </p>
                </section>

            </div>
        </ErrorBoundary>
    }
}
