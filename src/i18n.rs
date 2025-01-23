use fluent_templates::{static_loader, StaticLoader};
use leptos::prelude::*;
use leptos_fluent::leptos_fluent;
use std::sync::LazyLock;



static_loader! {
    static TRANSLATIONS = {
        locales: "./locales",
        fallback_language: "en",
    };
}

pub static COMPOUND: &[&LazyLock<StaticLoader>] =
    &[&TRANSLATIONS, &TRANSLATIONS];

#[component]
pub fn I18nProvider(children: Children) -> impl IntoView {
    leptos_fluent! {
        children: children(),
        translations: [TRANSLATIONS, TRANSLATIONS] + COMPOUND,
        locales: "./locales",
        check_translations: "./src/**/*.rs",
        sync_html_tag_lang: true,
        sync_html_tag_dir: true,
        cookie_name: "lang",
        cookie_attrs: "SameSite=Strict; Secure; path=/; max-age=600",
        initial_language_from_cookie: true,
        initial_language_from_cookie_to_localstorage: true,
        set_language_to_cookie: true,
        url_param: "lang",
        initial_language_from_url_param: true,
        initial_language_from_url_param_to_localstorage: true,
        initial_language_from_url_param_to_cookie: true,
        set_language_to_url_param: true,
        localstorage_key: "language",
        initial_language_from_localstorage: true,
        initial_language_from_localstorage_to_cookie: true,
        set_language_to_localstorage: true,
        initial_language_from_navigator: true,
        initial_language_from_navigator_to_localstorage: true,
        initial_language_from_accept_language_header: true,
    }
}
