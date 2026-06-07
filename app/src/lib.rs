use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <AutoReload options=options.clone() />
                <Stylesheet id="leptos" href="/pkg/colorful-nails.css" />
                <HydrationScripts options />
                <MetaTags />
            </head>
            <body class="bg-background">
                <App />
            </body>
        </html>
    }
}

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::booking::Booking;
use crate::pages::home::Home;
use crate::pages::services::Services;
use crate::pages::success::Success;

// Components
use crate::components::header::Header;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // sets the document title
        <Title text="Colorful Nails & Spa" />

        <Router>
            <Header />

            <Routes fallback=|| view! { NotFound }>
                <Route path=path!("/") view=Home />
                <Route path=path!("/services") view=Services />
                <Route path=path!("/booking") view=Booking />
                <Route path=path!("/success") view=Success />
            </Routes>
        </Router>
    }
}
