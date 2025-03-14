use leptos::prelude::*;
use leptos_icons::Icon;

#[component]
pub fn SummaryView() -> impl IntoView {
  view! {
    <div class="h-screen" id="summary">
      <h1>"CV"</h1>

      <h2>lyhyesti</h2>
      <div>jotain emt.</div>

      <div class="w-20 h-20">
        <Icon icon=icondata::BiChevronsDownRegular width="100%" height="100%" />
      </div>
    </div>
  }
}
