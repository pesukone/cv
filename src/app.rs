use crate::i18n::*;
use leptos::html;
use leptos::leptos_dom::logging::console_log;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
  components::{Route, Router, Routes, A},
  hooks::{use_navigate, use_query},
  params::Params,
  StaticSegment,
};
use std::collections::HashMap;
use std::iter::zip;
use std::time::Duration;

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
    <div class="flex flex-col gap-2 text-center">
      <h1>"CV"</h1>
      <SwitchLang />
      <ParamTest />
      <Skills />
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

fn display_class(class: &str) -> &str {
  match class {
    "hidden" => "hidden slide-2",
    "block" => "block slide-2",
    _ => "hidden slide-2",
  }
}

#[island]
fn Skills() -> impl IntoView {
  let (animating, set_animating) = signal(None);

  let skills = vec![
    "JavaScript",
    "TypeScript",
    "HTML",
    "CSS",
    "React",
    "Redux",
    "REST",
    "GraphQL",
    "Node.js",
    "SQL",
    "Docker",
    "Kubernetes",
    "Nix",
    "Ansible",
    "C++",
    "Go",
    "Rust",
  ];

  Effect::new(move |_| {
    set_timeout(
      move || {
        set_animating(Some(0));
      },
      Duration::from_secs(1),
    );
  });

  let skill_components = skills
    .iter()
    .enumerate()
    .map(|(idx, skill)| {
      view! {
        <div
          class=move || {
              display_class(
                  match animating() {
                      None => "hidden",
                      Some(i) => if i >= idx { "block" } else { "hidden" }
                  },
              )
          }
          on:animationend=move |_| set_animating(Some(idx + 1))
        >
          {skill.to_string()}
        </div>
      }
    })
    .collect_view();

  view! { <div class="flex flex-row gap-1">{skill_components}</div> }
}
