use crate::i18n::*;
use crate::skills::Skills;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes, A},
  hooks::use_query,
  params::Params,
  StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
  view! {
    <!DOCTYPE html>
    <html lang="en">
      <head>
        <meta charset="utf-8" />
        <meta name="viewport" content="width=device-width, initial-scale=1" />
        <AutoReload options=options.clone() />
        <HydrationScripts options islands=true />
        <MetaTags />
      </head>
      <body>
        <App />
      </body>
    </html>
  }
}

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    <Title text="CV" />

    <Stylesheet id="leptos" href="/pkg/cv.css" />

    // content for this welcome page
    <I18nContextProvider>
      <Router>
        <main>
          <Routes fallback=|| "Page not found.".into_view()>
            <Route path=StaticSegment("") view=CV />
          </Routes>
        </main>
      </Router>
    </I18nContextProvider>
  }
}

#[component]
fn CV() -> impl IntoView {
  view! {
    <div class="flex flex-col gap-2 items-center">
      <h1>"CV"</h1>
      <SwitchLang />
      <ParamTest />
      <Skills />
      <Certificates />
      <Education />
    </div>
  }
}

#[derive(Params, PartialEq)]
struct QueryParams {
  lang: Option<String>,
}

#[component]
fn ParamTest() -> impl IntoView {
  let params = use_query::<QueryParams>();
  let lang = params
    .read()
    .as_ref()
    .ok()
    .and_then(|params| params.lang.clone())
    .unwrap_or("fi".to_string());

  view! { <div>{lang}</div> }
}

#[component]
fn SwitchLang() -> impl IntoView {
  let params = use_query::<QueryParams>();
  let i18n = use_i18n();

  let lang = params
    .read()
    .as_ref()
    .ok()
    .and_then(|params| params.lang.clone())
    .unwrap_or_default();

  match lang.as_str() {
    "en" => i18n.set_locale(Locale::en),
    "fi" => i18n.set_locale(Locale::fi),
    _ => i18n.set_locale(Locale::fi),
  }

  let lang = match i18n.get_locale() {
    Locale::fi => "en",
    Locale::en => "fi",
  };

  view! {
    <A
      href=format!("/?lang={}", lang)
      attr:class="text-medium text-blue-600 dark:text-blue-500 hover:underline"
    >
      {t!(i18n, test_msg)}
    </A>
  }
}

#[component]
fn Certificates() -> impl IntoView {
  view! {
    <div class="w-1/3 border">
      <h2>Google cloud developer vai mikä olikaan</h2>
      <h3>6.9.2069</h3>
      <div>joku kuva/ikoni tähän</div>
    </div>
  }
}

#[component]
fn Education() -> impl IntoView {
  view! {
    <ul>
      <li>LuK, tietojenkäsittelytiede, Helsingin yliopisto x.x.2019</li>
      <li>Etelä-Tapiolan lukio x.x.2015</li>
    </ul>
  }
}
