use crate::i18n::*;
use crate::qualifications::FormalQualificationsView;
use crate::skills::Skills;
use crate::summary::SummaryView;
use leptos::prelude::*;
use leptos_icons::Icon;
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
    <div class="flex flex-col gap-2 items-center bg-neutral-100 scroll-smooth">
      <SwitchLang />
      <h1>CV</h1>

      <SummaryView />

      <Experience />
      <Skills />

      <FormalQualificationsView />

      <Hobbies />
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
      attr:class="flex border-2 items-center gap-1 p-1 rounded-lg cursor-pointer self-end fixed top-5 right-8"
    >
      <div class="w-4 h-4">
        <Icon icon=icondata::IoLanguage width="100%" height="100%" />
      </div>
      {lang}
    </A>
  }
}

#[component]
fn Experience() -> impl IntoView {
  view! {
    <h2>työkokemus</h2>
    <ol>
      <li>Ohjelmistokehittäjä, Taito United Oy 2019-</li>
      <li>tähän jotain prokkiksista ja teknologioista</li>
    </ol>
  }
}

#[component]
fn Languages() -> impl IntoView {
  view! {
    <h2>kielet</h2>
    <ul>
      <li>suomi - äidinkieli</li>
    </ul>
  }
}

#[component]
fn Contact() -> impl IntoView {
  view! {
    <h2>Jussi Aalto</h2>
    <div>sähköposti</div>
    <div>github jne.</div>
  }
}

#[component]
fn Hobbies() -> impl IntoView {
  view! {
    <h2>harrastuksia</h2>
    <ul>
      <li>kotipalvelimet</li>
      <li>pyöräily</li>
      <li>retkeily</li>
    </ul>
  }
}
