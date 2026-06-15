use leptos::prelude::*;

/// Skeleton loading state shaped like a booking confirmation card.
/// Shows while the bookings list is fetching.
#[component]
pub fn BookingCardSkeleton() -> impl IntoView {
    view! {
        <div class="skeleton-booking-card">
            // Service name + badge row
            <div class="flex items-center justify-between gap-4">
                <div class="skeleton-heading"></div>
                <div class="skeleton w-16 h-5 rounded-full shrink-0"></div>
            </div>
            // Date / time row
            <div class="flex flex-col gap-2">
                <div class="skeleton-label"></div>
                <div class="skeleton-line-sm"></div>
            </div>
            // Technician row
            <div class="flex flex-col gap-2">
                <div class="skeleton-label"></div>
                <div class="skeleton-line-sm"></div>
            </div>
            // Button row
            <div class="flex gap-3 mt-2">
                <div class="skeleton h-9 w-28 rounded-lg"></div>
                <div class="skeleton h-9 w-20 rounded-lg"></div>
            </div>
        </div>
    }
}

/// Skeleton loading state shaped like a service list row.
/// Shows while the services list is fetching.
#[component]
pub fn ServiceRowSkeleton() -> impl IntoView {
    view! {
        <div class="skeleton-service-row">
            <div class="skeleton-service-icon"></div>
            <div class="flex flex-col gap-1.5 flex-1">
                <div class="skeleton-line-sm"></div>
                <div class="skeleton h-3 w-1/3 rounded"></div>
            </div>
            <div class="skeleton h-5 w-14 rounded-full shrink-0"></div>
        </div>
    }
}

/// A generic text content skeleton: one heading + N body lines.
///
/// # Props
/// - `lines` — Number of body text lines to show (default: 3).
#[component]
pub fn TextSkeleton(#[prop(optional, default = 3)] lines: usize) -> impl IntoView {
    view! {
        <div class="flex flex-col gap-3">
            <div class="skeleton-heading"></div>
            {(0..lines)
                .map(|i| {
                    let w = if i == lines - 1 { "skeleton h-4 w-3/4 rounded" } else { "skeleton-line" };
                    view! { <div class=w></div> }
                })
                .collect_view()}
        </div>
    }
}
