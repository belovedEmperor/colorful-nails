use crate::components::errors::ErrorView;
use leptos::prelude::*;

struct ServiceCategory {
    name: &'static str,
    services: &'static [&'static str],
}

const SERVICE_CATEGORIES: &[ServiceCategory] = &[
    ServiceCategory {
        name: "Manicure & Pedicure",
        services: &["Manicure", "Gel Manicure", "Pedicure"],
    },
    ServiceCategory {
        name: "Nail Extras",
        services: &[
            "Nail Repair",
            "Nail Trim",
            "Nail
 Design",
            "Nail Removal",
        ],
    },
    ServiceCategory {
        name: "Acrylic",
        services: &[
            "Full Set",
            "Fill-ins",
            "Pink & White",
            "Fill-ins Pink & White",
        ],
    },
    ServiceCategory {
        name: "UV Gel",
        services: &[
            "UV Gel",
            "UV Gel Fill-ins",
            "UV Gel Pink &
 White",
        ],
    },
    ServiceCategory {
        name: "Polish",
        services: &[
            "French",
            "Hands Polish Change",
            "Toes Polish
 Change",
        ],
    },
    ServiceCategory {
        name: "Waxing",
        services: &[
            "Eyebrow",
            "Lip Wax",
            "Arm Wax",
            "Half Arm",
            "Leg Wax",
            "Underarm",
            "Bikini Wax",
            "Brazilian",
        ],
    },
];

#[component]
fn ServiceSection(
    service_categories: &'static [ServiceCategory],
    img_left: bool,
    img_src: &'static str,
    img_alt: &'static str,
) -> impl IntoView {
    let forward =
        "section-padding section-container flex flex-col lg:flex-row items-center justify-center";
    let reverse = "section-padding section-container flex flex-col lg:flex-row-reverse items-center justify-center";
    view! {
        <section class=if img_left { forward } else { reverse }>
            <img
                class="max-w-sm max-h-96 md:max-w-lg object-cover rounded-lg"
                src=img_src
                alt=img_alt
            />
            {service_categories
                .iter()
                .map(|service_category| {
                    view! {
                        <div class="section-container flex flex-col items-center justify-center w-full bg-primary rounded-lg p-6">
                            <h3 class="section-header">{service_category.name}</h3>
                            <ul class="divide-y divide-secondary w-full">
                                {service_category
                                    .services
                                    .iter()
                                    .map(|service| view! { <li class="py-2">{*service}</li> })
                                    .collect_view()}
                            </ul>
                        </div>
                    }
                })
                .collect_view()}
        </section>
    }
}

#[component]
pub fn Services() -> impl IntoView {
    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="flex flex-col page-container section-padding">
                <h1 class="page-header">"Services"</h1>
                <div class="flex flex-col">
                    <p class="self-end bg-secondary px-3 py-1 rounded-full">
                        "Gel polish is extra"
                    </p>
                    <ServiceSection
                        service_categories=&SERVICE_CATEGORIES[0..=1]
                        img_src="/manicure.jpg"
                        img_alt="Autumn nail art"
                        img_left=true
                    />
                    <ServiceSection
                        service_categories=&SERVICE_CATEGORIES[2..=4]
                        img_src="/acrylic.jpg"
                        img_alt="Acrylic nails with 3d acrylic flowers"
                        img_left=false
                    />
                    <section class="section-padding section-container flex items-center justify-center">
                        {SERVICE_CATEGORIES[5..=5]
                            .iter()
                            .map(|service_category| {
                                view! {
                                    <div class="section-container flex flex-col items-center justify-center w-full bg-primary rounded-lg p-6">
                                        <h3 class="section-header">{service_category.name}</h3>
                                        <ul class="w-full lg:grid lg:grid-cols-2 [&>li]:border-b [&>li]:border-secondary
                                        lg:[&>li:nth-last-child(-n+2)]:border-b-0">
                                            {service_category
                                                .services
                                                .iter()
                                                .map(|service| {
                                                    view! { <li class="py-2">{*service}</li> }
                                                })
                                                .collect_view()}
                                        </ul>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </section>
                </div>
            </div>
        </ErrorBoundary>
    }
}
