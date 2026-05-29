use crate::components::errors::ErrorView;
use leptos::prelude::*;

#[component]
pub fn Booking() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="flex flex-col">
                <h1 class="page-header">"Booking"</h1>
            </div>
        </ErrorBoundary>
    }
}
