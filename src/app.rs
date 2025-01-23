use fluent_templates::{static_loader, StaticLoader};
use leptos::prelude::*;
use leptos_fluent::{expect_i18n, move_tr, tr};
use leptos_meta::Title;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use std::sync::LazyLock;

use crate::i18n::I18n;



static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

pub static COMPOUND: &[&LazyLock<StaticLoader>] =
    &[&TRANSLATIONS, &TRANSLATIONS];


#[component]
pub fn App() -> impl IntoView {
    view! {
        <I18n>
            <Title text=move || tr!("welcome-to-leptos") />

            <Router>
                <main>
                    <Routes fallback=|| tr!("not-found").into_view()>
                        <Route path=StaticSegment("/") view=PageHome />
                    </Routes>
                </main>
            </Router>
        </I18n>
    }
}

#[component]
fn PageHome() -> impl IntoView {
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

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_axum::ResponseOptions>();
        resp.set_status(axum::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>{move_tr!("not-found")}</h1> }
}
