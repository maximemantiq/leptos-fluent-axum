use leptos::prelude::*;
use leptos_fluent::{expect_i18n, move_tr};



#[component]
pub fn PageHome() -> impl IntoView {
    let i18n = expect_i18n();

    view! {
        <h1>{move_tr!("select-a-language")}</h1>
        <fieldset>

            {move || {
                i18n.languages
                    .iter()
                    .map(|lang| {
                        view! {
                            <div>
                                <input
                                    type="radio"
                                    id=lang
                                    name="language"
                                    value=lang
                                    checked=lang.is_active()
                                    on:click=move |_| lang.activate()
                                />
                                <label for=lang>{lang.name}</label>
                            </div>
                        }
                    })
                    .collect::<Vec<_>>()
            }}

        </fieldset>
    }
}
