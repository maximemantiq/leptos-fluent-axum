use leptos::prelude::*;
use fluent_templates::{static_loader, StaticLoader};
use leptos_fluent::tr;
use leptos_meta::Title;
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};
use std::sync::LazyLock;

use crate::{i18n::I18nProvider, routing::page_home::PageHome};



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
        <I18nProvider>
            <Title text=move || tr!("welcome_to_leptos") />

            <Router>
                <main>
                    <Routes fallback=|| tr!("not_found").into_view()>
                        <Route path=StaticSegment("/") view=PageHome />
                    </Routes>
                </main>
            </Router>
        </I18nProvider>
    }
}

