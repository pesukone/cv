use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_use::use_window_scroll;

#[component]
pub fn SummaryView() -> impl IntoView {
  view! {
    <div class="flex flex-col items-center pb-8 w-full h-screen" id="summary">
      <h2>lyhyesti</h2>
      <div>jotain emt.</div>

      <div class="mt-auto">
        <Chevron />
      </div>
    </div>
  }
}

fn get_classes(animate: &str) -> &str {
  match animate {
    "in" => "w-20 h-20 animate-bounce animate-fade-in",
    "out" => "w-20 h-20 animate-bounce animate-fade-out",
    _ => "w-20 h-20 animate-bounce",
  }
}

#[island]
fn Chevron() -> impl IntoView {
  let (_x, y) = use_window_scroll();
  view! {
    <div class=move || { if y() > 0.0 { get_classes("out") } else { get_classes("in") } }>
      <Icon icon=icondata::IoChevronDown width="100%" height="100%" />
    </div>
  }
}
