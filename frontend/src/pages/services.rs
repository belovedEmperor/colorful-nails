use crate::components::errors::ErrorView;
use leptos::prelude::*;

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="flex flex-col">
                <h1 class="page-header">"Services"</h1>
                <div class="page-container grid grid-cols-2">
                    <section class="page-container flex flex-col justify-center items-center gap-8 p-24">
                        <h2 class="section-header">TEMP</h2>
                        <ul>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                        </ul>
                    </section>
                    <section class="page-container flex flex-col justify-center items-center gap-8 p-24">
                        <h2 class="section-header">TEMP</h2>
                        <ul>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                        </ul>
                    </section>
                    <section class="page-container flex flex-col justify-center items-center gap-8 p-24">
                        <h2 class="section-header">TEMP</h2>
                        <ul>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                        </ul>
                    </section>
                    <section class="page-container flex flex-col justify-center items-center gap-8 p-24">
                        <h2 class="section-header">TEMP</h2>
                        <ul>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                            <li>"Temp service"</li>
                        </ul>
                    </section>
                </div>
            </div>
        </ErrorBoundary>
    }
}
