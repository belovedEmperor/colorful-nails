use crate::components::errors::ErrorView;
use leptos::prelude::*;

#[component]
pub fn Booking() -> impl IntoView {
    let (phone_number, set_phone_number) = signal("".to_string());

    view! {
        <ErrorBoundary fallback=|errors| {
            view! { <ErrorView errors=errors /> }
        }>
            <div class="flex flex-col">
                <h1 class="page-header">"Booking"</h1>

                <form action="post">
                    <label for="first_name">"First Name:*"</label>
                    <input
                        type="text"
                        placeholder="Jane"
                        id="first_name"
                        name="first_name"
                        required
                    />
                    <label for="last_name">"Last Name:"</label>
                    <input type="text" placeholder="Doe" id="last_name" name="last_name" />

                    <label for="phone_number">"Phone Number:*"</label>
                    <input
                        type="tel"
                        placeholder="(570)999-9999"
                        id="last_name"
                        name="last_name"
                        pattern="\\([0-9]{3}\\)[0-9]{3}-[0-9]{4}"
                        required
                        on:input:target=move |event| {
                            set_phone_number.set(format_phone_number(event.target().value()))
                        }
                        prop:value=phone_number
                    />

                    <label for="email">"Email:*"</label>
                    <input
                        type="email"
                        placeholder="example@example.com"
                        id="email"
                        name="email"
                        required
                    />

                    <label for="services">"Services:"</label>
                    <select></select>

                    <label for="preferred_date">"Preferred Date:*"</label>
                    <input type="radio" id="preferred_date" name="preferred_date" required />

                    <label for="preferred_time">"Preferred Time:*"</label>
                    <input type="radio" id="preferred_time" name="preferred_time" required />

                    <label for="notes">"Notes:"</label>
                    <textarea type="text" id="notes" name="notes" rows="5" columns="33" />

                    <input type="submit" value="Submit" />
                </form>
                <p>{phone_number}</p>
            </div>
        </ErrorBoundary>
    }
}

fn format_phone_number(input: String) -> String {
    let digits: String = input.chars().filter(|char| char.is_ascii_digit()).collect();

    match digits.len() {
        0 => "".to_string(),
        1..=3 => format!("({digits})"),
        4..=6 => {
            let (area_code, rest) = digits.split_at(3);
            format!("({area_code})-{rest}")
        }
        7..=10 => {
            let (area_code, rest) = digits.split_at(3);
            let (middle, rest) = rest.split_at(3);
            format!("({area_code})-{middle}-{rest}")
        }
        _ => {
            let (area_code, rest) = digits.split_at(3);
            let (middle, rest) = rest.split_at(3);
            let (rest, _) = rest.split_at(4);
            format!("({area_code})-{middle}-{rest}")
        }
    }
}
