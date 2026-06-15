use leptos::prelude::*;

/// Labeled text input with optional hint and error message.
///
/// # Props
/// - `id`          — HTML `id` (also used for `for` on the label).
/// - `label`       — Label text shown above the input.
/// - `input_type`  — HTML `type`, e.g. `"text"`, `"email"`, `"tel"`, `"datetime-local"`. Default: `"text"`.
/// - `name`        — HTML `name` attribute (required for form submission).
/// - `value`       — Controlled value string.
/// - `placeholder` — Placeholder text.
/// - `hint`        — Small helper text below the input.
/// - `error`       — Error message; when non-empty the input gets error styling.
/// - `required`    — Marks the field required.
/// - `disabled`    — Disables the input.
/// - `class`       — Additional classes on the outer container.
#[component]
pub fn TextInput(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] input_type: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] value: String,
    #[prop(optional, into)] placeholder: String,
    #[prop(optional, into)] hint: String,
    #[prop(optional, into)] error: String,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let input_type = if input_type.is_empty() {
        "text".to_owned()
    } else {
        input_type
    };

    let has_error = !error.is_empty();
    let has_hint = !hint.is_empty();
    let has_label = !label.is_empty();

    let input_class = if has_error {
        "input input-error"
    } else {
        "input"
    };

    let container_class = format!("input-container {class}");

    // Pre-clone id so each use in the view gets its own owned String.
    let id_for_label = id.clone();
    let id_for_input = id;

    view! {
        <div class=container_class>
            <Show when=move || has_label>
                <label class="input-label" for=id_for_label.clone()>
                    {label.clone()}
                </label>
            </Show>
            <input
                class=input_class
                id=id_for_input.clone()
                type=input_type
                name=name
                value=value
                placeholder=placeholder
                required=required
                disabled=disabled
            />
            <Show when=move || has_error>
                <p class="input-error-text">{error.clone()}</p>
            </Show>
            <Show when=move || has_hint && !has_error>
                <p class="input-hint">{hint.clone()}</p>
            </Show>
        </div>
    }
}

/// Native `<select>` styled to match `.input`.
///
/// Pass `<option>` elements as children.
///
/// # Example
/// ```rust
/// <SelectInput id="service" label="Service" name="service">
///     <option value="">"Choose a service..."</option>
///     <option value="manicure">"Manicure — $25"</option>
///     <option value="pedicure">"Pedicure — $35"</option>
/// </SelectInput>
/// ```
#[component]
pub fn SelectInput(
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] hint: String,
    #[prop(optional, into)] error: String,
    #[prop(optional)] required: bool,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    let has_error = !error.is_empty();
    let has_hint = !hint.is_empty();
    let has_label = !label.is_empty();

    let input_class = if has_error {
        "input input-error"
    } else {
        "input"
    };

    let container_class = format!("input-container {class}");

    let id_for_label = id.clone();
    let id_for_select = id;

    view! {
        <div class=container_class>
            <Show when=move || has_label>
                <label class="input-label" for=id_for_label.clone()>
                    {label.clone()}
                </label>
            </Show>
            <select
                class=input_class
                id=id_for_select.clone()
                name=name
                required=required
                disabled=disabled
            >
                {children()}
            </select>
            <Show when=move || has_error>
                <p class="input-error-text">{error.clone()}</p>
            </Show>
            <Show when=move || has_hint && !has_error>
                <p class="input-hint">{hint.clone()}</p>
            </Show>
        </div>
    }
}

/// Accessible toggle switch (coral when on).
///
/// # Props
/// - `checked`   — Reactive read signal driving the checked state.
/// - `on_change` — Called with the new bool when toggled. Must be `Send + Sync`.
/// - `label`     — Visible label next to the toggle.
/// - `id`        — HTML id for the hidden checkbox.
///
/// # Note
/// The `Send + Sync` bound on `on_change` is required by Leptos 0.8's `StoredValue`.
/// In practice this means the closure cannot capture `Rc` or `RefCell` — use signals instead.
#[component]
pub fn Toggle(
    checked: ReadSignal<bool>,
    on_change: impl Fn(bool) + Send + Sync + 'static,
    #[prop(optional, into)] label: String,
    #[prop(optional, into)] id: String,
) -> impl IntoView {
    let on_change = store_value(on_change);
    let has_label = !label.is_empty();

    view! {
        <label class="flex items-center gap-3 cursor-pointer select-none">
            <input
                type="checkbox"
                id=id
                class="sr-only"
                checked=move || checked.get()
                on:change=move |_| on_change.with_value(|f| f(!checked.get()))
            />
            <div
                class="toggle-track"
                data-checked=move || if checked.get() { "true" } else { "false" }
            >
                <div
                    class="toggle-thumb"
                    data-checked=move || if checked.get() { "true" } else { "false" }
                ></div>
            </div>
            <Show when=move || has_label>
                <span class="text-sm font-medium text-midnight-ink font-sans">
                    {label.clone()}
                </span>
            </Show>
        </label>
    }
}
