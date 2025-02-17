use crate::i18n::*;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes},
  StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
      <!DOCTYPE html>
      <html lang="en">
          <head>
              <meta charset="utf-8"/>
              <meta name="viewport" content="width=device-width, initial-scale=1"/>
              <AutoReload options=options.clone() />
              <HydrationScripts options islands=true/>
              <MetaTags/>
          </head>
          <body>
              <App/>
          </body>
      </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href="/pkg/cv.css"/>

    // sets the document title
    <Title text="CV"/>

    // content for this welcome page
    <I18nContextProvider>
      <Router>
        <main>
          <Routes fallback=|| "Page not found.".into_view()>
            <Route path=StaticSegment("") view=CV/>
          </Routes>
        </main>
      </Router>
    </I18nContextProvider>
  }
}

#[component]
fn CV() -> impl IntoView {
  view! {
    <h1>"CV"</h1>
    <SwitchLang/>
  }
}

#[island]
fn SwitchLang() -> impl IntoView {
  let i18n = use_i18n();

  let on_switch = move |_| {
    let new_lang = match i18n.get_locale() {
      Locale::fi => Locale::en,
      Locale::en => Locale::fi,
    };
    i18n.set_locale(new_lang);
  };

  view! {
    <button on:click=on_switch>{t!(i18n, test_msg)}</button>
  }
}
