use crate::components::{errors::ErrorView, nav_btn::NavButton};
use leptos::prelude::*;

/// Success Page
#[component]
pub fn Success() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="section-padding section-container page-container justify-center section-container flex flex-col">
                <h2 class="section-header">"Your Appointment Is Made!"</h2>
                <p>"We will email you about the status of the appointment soon."</p>
                <NavButton button_class="bg-primary button" href="/booking" text_content="Back" />
            </div>
        </ErrorBoundary>
    }
}
